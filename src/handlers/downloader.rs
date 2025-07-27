use yt_dlp::Youtube;
use std::path::PathBuf;
use uuid::Uuid;
use yt_dlp::fetcher::download_manager::ManagerConfig;
use yt_dlp::fetcher::deps::Libraries;
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
    
    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");
    
    let libraries = Libraries::new(youtube, ffmpeg);
    
    let fetcher = Youtube::with_download_manager_config(libraries, output_dir, config)?;

    Ok(fetcher)
}

pub async fn start_background_download(url: String, job_id: String) {
    // TODO
}
