use yt_dlp::Youtube;
use std::path::PathBuf;
use uuid::Uuid;
use yt_dlp::fetcher::download_manager::ManagerConfig;
use yt_dlp::fetcher::deps::{Libraries, LibraryInstaller};
use crate::config::Config;

#[derive(Debug)]
pub struct DownloadJob {
    job_id: String,
    status: bool,
    file_path: PathBuf,
}

#[derive(Debug)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Error,
}

pub async fn init_yt_dlp() -> Result<Youtube, Box<dyn std::error::Error>> {
    let config = ManagerConfig {
        max_concurrent_downloads: 5,        // Maximum 5 concurrent downloads
        segment_size: 1024 * 1024 * 10,     // 10 MB per segment
        parallel_segments: 8,               // 8 parallel segments per download
        retry_attempts: 5,                  // 5 retry attempts on failure
        max_buffer_size: 1024 * 1024 * 20,  // 20 MB maximum buffer
    };

    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("downloads");
    
    let installer = LibraryInstaller::new(libraries_dir.clone());
    
    let youtube = installer.install_youtube(None).await?;
    
    let ffmpeg = installer.install_ffmpeg(None).await?;
    
    let libraries = Libraries::new(youtube, ffmpeg);
    
    let fetcher = Youtube::with_download_manager_config(libraries, output_dir, config)?;

    Ok(fetcher)
}

pub async fn download_video(url: String, job_id: String) -> Result<PathBuf, Box<dyn std::error::Error>> {
    println!("Starting download for URL: {}", url);
    
    let config = Config::new();

    let fetcher = init_yt_dlp().await?;

    let video = fetcher.fetch_video_infos(url.clone()).await?;
    println!("Video info fetched successfully");

    let video_path = fetcher.download_video_with_quality(
        url.clone(),
        format!("{}.mp4", job_id),
        config.video_quality,
        config.video_codec,
        config.audio_quality,
        config.audio_codec
    ).await?;

    println!("Download completed: {:?}", video_path);

    Ok(video_path)
}

pub async fn start_background_download(url: String, job_id: String) {
    match download_video(url, job_id).await {
        Ok(path) => println!("Download completed: {:?}", path),
        Err(e) => println!("Download failed: {:?}", e),
    }
}
