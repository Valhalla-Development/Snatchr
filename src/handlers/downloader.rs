use crate::config::Config;
use std::path::PathBuf;
use yt_dlp::Youtube;
use yt_dlp::fetcher::deps::{Libraries, LibraryInstaller};
use yt_dlp::fetcher::download_manager::ManagerConfig;
extern crate sanitize_filename;
use std::time::{Duration, Instant};
use tracing::{error, info};

pub fn init_yt_dlp() -> Result<Youtube, Box<dyn std::error::Error>> {
    let app_config = Config::from_env();

    let config = ManagerConfig {
        max_concurrent_downloads: app_config.max_concurrent_downloads,
        segment_size: 1024 * 1024 * 10,    // 10 MB per segment
        parallel_segments: 8,              // 8 parallel segments per download
        retry_attempts: 5,                 // 5 retry attempts on failure
        max_buffer_size: 1024 * 1024 * 20, // 20 MB maximum buffer
    };

    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from(&app_config.download_dir);

    // Use blocking runtime for sync operations
    let rt = tokio::runtime::Runtime::new()?;

    let (youtube, ffmpeg) = rt.block_on(async {
        let installer = LibraryInstaller::new(libraries_dir.clone());
        let youtube = installer.install_youtube(None).await?;
        let ffmpeg = installer.install_ffmpeg(None).await?;
        Ok::<_, Box<dyn std::error::Error>>((youtube, ffmpeg))
    })?;

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::with_download_manager_config(libraries, output_dir, config)?;

    Ok(fetcher)
}

pub fn download_video(
    url: String,
    job_id: String,
) -> Result<(PathBuf, Duration), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let config = Config::from_env();

    info!(job_id = %job_id, url = %url, "Starting download job");

    let fetcher = match init_yt_dlp() {
        Ok(f) => f,
        Err(e) => {
            error!(job_id = %job_id, url = %url, error = %e, "Failed to initialize yt-dlp");
            return Err(e);
        }
    };

    // Create runtime for async operations within sync context
    let rt = tokio::runtime::Runtime::new()?;

    let result = rt.block_on(async {
        info!(job_id = %job_id, url = %url, "Fetching video info");
        let video = fetcher.fetch_video_infos(url.clone()).await?;

        info!(job_id = %job_id, url = %url, video_title = %video.title, "Video info fetched");

        let job_dir = PathBuf::from(&config.download_dir).join(&job_id);
        std::fs::create_dir_all(&job_dir)?;
        info!(job_id = %job_id, url = %url, path = %job_dir.display(), "Created job directory");

        let filename = format!("{}.mp4", video.title);
        let relative_path = format!("{}/{}", job_id, sanitize_filename::sanitize(&filename));

        info!(
            job_id = %job_id,
            url = %url,
            video_title = %video.title,
            quality = ?config.video_quality,
            video_codec = ?config.video_codec,
            audio_quality = ?config.audio_quality,
            audio_codec = ?config.audio_codec,
            "Starting download with specified quality and codecs"
        );

        let video_path = fetcher
            .download_video_with_quality(
                url.clone(),
                relative_path,
                config.video_quality,
                config.video_codec,
                config.audio_quality,
                config.audio_codec,
            )
            .await?;

        Ok::<_, Box<dyn std::error::Error>>((video, video_path))
    });

    let duration = start.elapsed();

    match result {
        Ok((_video, video_path)) => {
            info!(
                job_id = %job_id,
                url = %url,
                path = %video_path.display(),
                duration = format_args!("{:.2}s", duration.as_secs_f64()),
                "Download completed successfully"
            );
            Ok((video_path, duration))
        }
        Err(e) => {
            error!(
                job_id = %job_id,
                url = %url,
                error = %e,
                duration = format_args!("{:.2}s", duration.as_secs_f64()),
                "Download failed"
            );
            Err(e)
        }
    }
}
