use crate::config::Config;
use crate::utils::video_id::extract_cache_id;
use std::path::{Path, PathBuf};
use yt_dlp::Downloader;
extern crate sanitize_filename;
use std::cell::RefCell;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

const MIN_VALID_VIDEO_SIZE_BYTES: u64 = 1024;

/*
 * Initializes the multi-platform yt-dlp downloader.
 * Reuses existing yt-dlp and ffmpeg binaries, installing only missing binaries.
 * Returns a configured fetcher ready to download videos.
 */
pub fn init_yt_dlp() -> Result<Downloader, Box<dyn std::error::Error>> {
    let app_config = Config::from_env();
    let libraries_dir = PathBuf::from("libs"); // Directory for external libs
    let output_dir = PathBuf::from(&app_config.download_dir); // Directory for downloads

    // Create a Tokio runtime to initialize the async downloader in a blocking context
    let rt = tokio::runtime::Runtime::new()?;

    let fetcher = rt.block_on(async {
        Downloader::with_new_binaries(libraries_dir, output_dir)
            .await?
            .with_timeout(Duration::from_secs(app_config.timeout_seconds))
            .with_max_concurrent_downloads(app_config.max_concurrent_downloads)
            .build()
            .await
    })?;
    Ok(fetcher)
}

/// True when a file is a finished cache entry (not an in-progress temp download).
fn is_final_cached_mp4(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    // In-progress downloads use hidden names like ".Title.abcd1234.tmp.mp4"
    path.extension().is_some_and(|ext| ext == "mp4") && !name.starts_with('.')
}

/// Looks for a reusable mp4 under downloads/{video_id}/.
/// Removes tiny/poisoned final files so the next download can replace them.
fn find_cached_mp4(download_dir: &Path, video_id: &str) -> Option<PathBuf> {
    let cache_dir = download_dir.join(video_id);
    if !cache_dir.is_dir() {
        return None;
    }

    let entries = std::fs::read_dir(&cache_dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() || !is_final_cached_mp4(&path) {
            continue;
        }

        match std::fs::metadata(&path) {
            Ok(metadata) if metadata.len() >= MIN_VALID_VIDEO_SIZE_BYTES => {
                return Some(path);
            }
            Ok(_) => {
                // Tiny files are usually error/anti-bot payloads — drop them.
                let _ = std::fs::remove_file(&path);
            }
            Err(_) => {}
        }
    }
    None
}

/// Drops leftover in-progress downloads in a video cache directory.
fn scrub_incomplete_downloads(cache_dir: &Path) {
    let Ok(entries) = std::fs::read_dir(cache_dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if path.is_file() && name.starts_with('.') && name.ends_with(".tmp.mp4") {
            let _ = std::fs::remove_file(&path);
        }
    }
}

/// Publishes a completed download by renaming the temp file into place.
/// Rejects undersized outputs so they never become cache hits.
fn publish_completed_download(
    temp_path: &Path,
    final_path: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let size = std::fs::metadata(temp_path)?.len();
    if size < MIN_VALID_VIDEO_SIZE_BYTES {
        let _ = std::fs::remove_file(temp_path);
        return Err(format!(
            "Downloaded file too small ({size} bytes) — refusing to cache"
        )
        .into());
    }

    if final_path.exists() {
        std::fs::remove_file(final_path)?;
    }
    std::fs::rename(temp_path, final_path)?;
    Ok(final_path.to_path_buf())
}

/*
 * Downloads a video from the given URL and associates it with a job ID.
 * Measures download duration, logs progress and errors.
 * Creates job-specific directory, sanitizes filenames, and uses quality and codec config.
 * Returns the final path of the downloaded file and the duration taken.
 */
pub fn download_video(
    url: String,
    job_id: String,
) -> Result<(PathBuf, Duration), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let config = Config::from_env();

    info!(job = %job_id, url = %url, "Job started");

    // Check if download directory exists
    let download_dir = PathBuf::from(&config.download_dir);
    if !download_dir.exists() {
        error!(job = %job_id, dir = %download_dir.display(), "Download directory missing");
        return Err(format!(
            "Download directory does not exist: {}. Please create the directory or configure the DOWNLOAD_DIR environment variable.",
            download_dir.display()
        )
        .into());
    }

    // Fast path: parse the platform id from the URL and reuse a cached file
    // without initializing yt-dlp or fetching metadata.
    if let Some(cache_id) = extract_cache_id(&url) {
        if let Some(path) = find_cached_mp4(&download_dir, &cache_id) {
            let duration = start.elapsed();
            info!(
                job = %job_id,
                video = %cache_id,
                path = %path.display(),
                took = format_args!("{:.2}s", duration.as_secs_f64()),
                "Cache hit, skipped metadata"
            );
            return Ok((path, duration));
        }
    }

    // Initialize yt-dlp fetcher with corruption handling
    let fetcher = match init_yt_dlp() {
        Ok(f) => f,
        Err(e)
            if e.to_string().contains("invalid Zip archive")
                || e.to_string().contains("Could not find EOCD") =>
        {
            warn!(job = %job_id, error = %e, "Corrupted yt-dlp libraries detected — reinstalling");

            // Remove corrupted libs directory
            let libs_dir = PathBuf::from("libs");
            if libs_dir.exists() {
                std::fs::remove_dir_all(&libs_dir).unwrap_or_else(|e| {
                    error!(error = %e, "Failed to remove corrupted libs directory");
                });
                info!("Removed corrupted libs, retrying initialization");
            }

            // Retry initialization
            match init_yt_dlp() {
                Ok(f) => f,
                Err(retry_error) => {
                    error!(job = %job_id, error = %retry_error, "yt-dlp init failed after cleanup");
                    return Err(retry_error);
                }
            }
        }
        Err(e) => {
            error!(job = %job_id, error = %e, "yt-dlp init failed");
            return Err(e);
        }
    };

    // Create a runtime to run async video info fetching and downloading
    let rt = tokio::runtime::Runtime::new()?;

    // Store video_id
    let cached_video_id = RefCell::new(Option::<String>::None);

    let result = rt.block_on(async {
        info!(job = %job_id, "Fetching metadata");
        let video = fetcher.fetch_video_infos(url.clone()).await?;

        info!(job = %job_id, title = %video.title, "Metadata fetched");

        // Fallback cache check using yt-dlp's canonical id (covers short links
        // and URL shapes we couldn't parse up front).
        let video_id = &video.id;
        *cached_video_id.borrow_mut() = Some(video_id.clone());
        if let Some(path) = find_cached_mp4(&download_dir, video_id) {
            info!(
                job = %job_id,
                video = %video_id,
                path = %path.display(),
                "Cache hit after metadata"
            );
            return Ok(path);
        }

        // Cache miss — download into downloads/{video_id}/
        let cache_dir = download_dir.join(video_id);
        std::fs::create_dir_all(&cache_dir)?;
        scrub_incomplete_downloads(&cache_dir);
        debug!(job = %job_id, dir = %cache_dir.display(), "Cache directory created");

        // Helper function to clean the filename
        fn clean(filename: &str) -> String {
            filename
                .trim()
                .chars()
                .filter_map(|c| match c {
                    c if c.is_alphanumeric() => Some(c),
                    ' ' | '-' | '_' => Some('_'), // Normalize separators to underscores
                    _ => None,                    // Remove invalid characters
                })
                .collect::<String>()
                .chars()
                .fold(String::new(), |mut acc, c| {
                    // Prevent consecutive underscores
                    if c == '_' && acc.ends_with('_') {
                        acc
                    } else {
                        acc.push(c);
                        acc
                    }
                })
                .trim_matches('_') // Remove leading/trailing underscores
                .to_string()
        }

        // Sanitize filename to avoid illegal characters
        let clean_title = clean(&sanitize_filename::sanitize(&video.title));
        let clean_title = if clean_title.is_empty() {
            "video".to_string()
        } else {
            clean_title
        };
        // Keep a real .mp4 extension so remux/ffmpeg behave, but hide the name
        // so find_cached_mp4 ignores it until we atomically publish.
        let job_suffix = job_id.get(..8).unwrap_or("download");
        let temp_relative = format!("{video_id}/.{clean_title}.{job_suffix}.tmp.mp4");
        let final_relative = format!("{video_id}/{clean_title}.mp4");
        let final_path = download_dir.join(&final_relative);

        info!(
            job = %job_id,
            video = %video_id,
            quality = ?config.video_quality,
            vcodec = ?config.video_codec,
            acodec = ?config.audio_codec,
            "Downloading"
        );

        // Download to a temp name, then rename into place only if it looks valid.
        let temp_path = match fetcher
            .download(&video, &temp_relative)
            .video_quality(config.video_quality)
            .video_codec(config.video_codec.clone())
            .audio_quality(config.audio_quality)
            .audio_codec(config.audio_codec.clone())
            .execute()
            .await
        {
            Ok(path) => path,
            Err(e) => {
                let leftover = download_dir.join(&temp_relative);
                let _ = std::fs::remove_file(&leftover);
                return Err(e.into());
            }
        };

        let published = match publish_completed_download(&temp_path, &final_path) {
            Ok(path) => path,
            Err(e) => {
                let _ = std::fs::remove_file(&temp_path);
                return Err(e);
            }
        };

        Ok::<_, Box<dyn std::error::Error>>(published)
    });

    let duration = start.elapsed();

    // Log and return results based on success or failure
    match result {
        Ok(video_path) => {
            let video_id_borrowed = cached_video_id.borrow();
            let video_id_log = video_id_borrowed.as_deref().unwrap_or("unknown");
            info!(
                job = %job_id,
                video = %video_id_log,
                path = %video_path.display(),
                took = format_args!("{:.2}s", duration.as_secs_f64()),
                "Download complete"
            );
            Ok((video_path, duration))
        }
        Err(e) => {
            error!(
                job = %job_id,
                error = %e,
                took = format_args!("{:.2}s", duration.as_secs_f64()),
                "Download failed"
            );
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{OsStr, OsString};
    use std::fs;
    use uuid::Uuid;

    struct TempDownloadDir(PathBuf);

    impl TempDownloadDir {
        fn new() -> Self {
            let path =
                std::env::temp_dir().join(format!("snatchr-youtube-test-{}", Uuid::new_v4()));
            fs::create_dir_all(&path).expect("temporary download directory should be created");
            Self(path)
        }
    }

    impl Drop for TempDownloadDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    struct EnvVarGuard {
        key: &'static str,
        previous: Option<OsString>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: impl AsRef<OsStr>) -> Self {
            let previous = std::env::var_os(key);

            // This ignored test must be run by itself with one test thread.
            unsafe {
                std::env::set_var(key, value);
            }

            Self { key, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            unsafe {
                if let Some(previous) = &self.previous {
                    std::env::set_var(self.key, previous);
                } else {
                    std::env::remove_var(self.key);
                }
            }
        }
    }

    #[test]
    fn find_cached_mp4_ignores_incomplete_temp_files() {
        let download_dir = TempDownloadDir::new();
        let video_id = "abc123";
        let video_dir = download_dir.0.join(video_id);
        fs::create_dir_all(&video_dir).unwrap();

        fs::write(
            video_dir.join(".Title.job12345.tmp.mp4"),
            vec![0_u8; MIN_VALID_VIDEO_SIZE_BYTES as usize * 10],
        )
        .unwrap();

        assert!(find_cached_mp4(&download_dir.0, video_id).is_none());

        let final_path = video_dir.join("Title.mp4");
        fs::write(&final_path, vec![0_u8; MIN_VALID_VIDEO_SIZE_BYTES as usize]).unwrap();
        assert_eq!(find_cached_mp4(&download_dir.0, video_id), Some(final_path));
    }

    #[test]
    fn publish_rejects_undersized_downloads() {
        let download_dir = TempDownloadDir::new();
        let temp = download_dir.0.join(".video.tmp.mp4");
        let final_path = download_dir.0.join("video.mp4");
        fs::write(&temp, b"tiny").unwrap();

        let err = publish_completed_download(&temp, &final_path).unwrap_err();
        assert!(err.to_string().contains("too small"));
        assert!(!temp.exists());
        assert!(!final_path.exists());
    }

    #[test]
    fn publish_renames_valid_download_into_place() {
        let download_dir = TempDownloadDir::new();
        let temp = download_dir.0.join(".video.tmp.mp4");
        let final_path = download_dir.0.join("video.mp4");
        fs::write(&temp, vec![0_u8; MIN_VALID_VIDEO_SIZE_BYTES as usize]).unwrap();

        let published = publish_completed_download(&temp, &final_path).unwrap();
        assert_eq!(published, final_path);
        assert!(final_path.exists());
        assert!(!temp.exists());
    }

    #[test]
    fn cache_hit_from_url_skips_yt_dlp() {
        let download_dir = TempDownloadDir::new();
        let video_id = "dQw4w9WgXcQ";
        let video_dir = download_dir.0.join(video_id);
        fs::create_dir_all(&video_dir).expect("cache directory should be created");

        let cached_file = video_dir.join("cached.mp4");
        fs::write(&cached_file, vec![0_u8; MIN_VALID_VIDEO_SIZE_BYTES as usize])
            .expect("cached file should be written");

        let _download_dir = EnvVarGuard::set("DOWNLOAD_DIR", &download_dir.0);
        let (path, _duration) = download_video(
            format!("https://www.youtube.com/watch?v={video_id}"),
            "cache-hit-test".to_string(),
        )
        .expect("cached video should be returned without downloading");

        assert_eq!(path, cached_file);
    }

    #[test]
    #[ignore = "downloads a real YouTube video; run manually"]
    fn downloads_real_youtube_video() {
        let download_dir = TempDownloadDir::new();
        let _download_dir = EnvVarGuard::set("DOWNLOAD_DIR", &download_dir.0);
        let _video_quality = EnvVarGuard::set("VIDEO_QUALITY", "Low");
        let _video_codec = EnvVarGuard::set("VIDEO_CODEC", "any");
        let _audio_quality = EnvVarGuard::set("AUDIO_QUALITY", "Low");
        let _audio_codec = EnvVarGuard::set("AUDIO_CODEC", "any");
        let url = std::env::var("SNATCHR_TEST_YOUTUBE_URL")
            .unwrap_or_else(|_| "https://www.youtube.com/watch?v=tCDvOQI3pco".to_string());

        let (path, duration) = download_video(url, "youtube-smoke-test".to_string())
            .expect("YouTube video should download successfully");
        let metadata = fs::metadata(&path).expect("downloaded video should exist");

        assert!(path.starts_with(&download_dir.0));
        assert!(metadata.is_file());
        assert!(metadata.len() >= MIN_VALID_VIDEO_SIZE_BYTES);
        assert!(!duration.is_zero());
    }
}
