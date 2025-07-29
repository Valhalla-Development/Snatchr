/*
 * HTTP handler for download requests using Axum framework.
 *
 * Expects a JSON payload with a 'url' field representing the video URL to download.
 *
 * Steps:
 * 1. Generate a unique job ID for tracking the download.
 * 2. Clone the URL from the request payload.
 * 3. Offload the blocking download operation to a dedicated thread using `spawn_blocking`.
 * 4. Handle any errors during task execution or download process.
 * 5. Construct a public-facing file URL by stripping the download directory prefix.
 * 6. Return a JSON response containing the relative URL to the downloaded file.
 */

use axum::{Json, response::Result};
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, timeout};
use uuid::Uuid;

use crate::config::Config;
use crate::handlers::downloader::download_video;

#[derive(Deserialize)]
pub struct DownloadRequest {
    url: String,
}

#[derive(Serialize)]
pub struct DownloadResponse {
    success: bool,
    file_url: String,
}

#[axum::debug_handler]
pub async fn download_handler(
    Json(payload): Json<DownloadRequest>,
) -> Result<Json<DownloadResponse>> {
    let job_id = Uuid::new_v4().to_string();
    let url = payload.url.clone();
    let config = Config::from_env();

    // Run the download_video function on a blocking thread since it performs sync operations
    let (file_path, _duration) = timeout(
        Duration::from_secs(config.timeout_seconds as u64),
        tokio::task::spawn_blocking(move || download_video(url, job_id).map_err(|e| e.to_string())),
    )
    .await
    .map_err(|_| "Download timeout".to_string())? // Handle timeout errors
    .map_err(|e| format!("Task join error: {}", e))? // Handle task join errors
    .map_err(|e| format!("Download error: {}", e))?; // Handle download errors

    // Create a relative file URL by stripping the base download directory from the absolute path
    let file_url = format!(
        "/{}",
        file_path
            .strip_prefix(&config.download_dir)
            .unwrap_or(&file_path)
            .to_string_lossy()
    );

    Ok(Json(DownloadResponse {
        success: true,
        file_url,
    }))
}
