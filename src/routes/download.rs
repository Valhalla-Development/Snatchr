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

use axum::Json;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, timeout};
use tracing::error;
use urlencoding::encode;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    file_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[axum::debug_handler]
pub async fn download_handler(Json(payload): Json<DownloadRequest>) -> Json<DownloadResponse> {
    let job_id = Uuid::new_v4().to_string();
    let url = payload.url.clone();
    let config = Config::from_env();

    // Validate YouTube URL format
    let youtube_regex = Regex::new(r"https?://(?:www\.|m\.)?youtube\.com/(?:watch\?v=|shorts/)[^\s]+|https?://youtu\.be/[^\s]+").unwrap();

    if !youtube_regex.is_match(&url) {
        return Json(DownloadResponse {
            success: false,
            file_url: None,
            error: Some("Invalid YouTube URL format".to_string()),
        });
    }

    // Run the download_video function on a blocking thread since it performs sync operations
    let job_id_clone = job_id.clone();
    let result = timeout(
        Duration::from_secs(config.timeout_seconds as u64),
        tokio::task::spawn_blocking(move || download_video(url, job_id).map_err(|e| e.to_string())),
    )
    .await;

    let (file_path, _duration) = match result {
        Ok(task_result) => match task_result {
            Ok(download_result) => match download_result {
                Ok(result) => result,
                Err(e) => {
                    error!(job_id = %job_id_clone, error = %e, "Download error occurred");
                    return Json(DownloadResponse {
                        success: false,
                        file_url: None,
                        error: Some(format!("Download error: {}", e)),
                    });
                }
            },
            Err(e) => {
                error!(job_id = %job_id_clone, error = %e, "Task join error occurred");
                return Json(DownloadResponse {
                    success: false,
                    file_url: None,
                    error: Some(format!("Task join error: {}", e)),
                });
            }
        },
        Err(_) => {
            error!(job_id = %job_id_clone, "Download timeout occurred");
            return Json(DownloadResponse {
                success: false,
                file_url: None,
                error: Some("Download timeout".to_string()),
            });
        }
    };

    // Create a full file URL pointing to our file serving endpoint
    let relative_path = file_path
        .strip_prefix(&config.download_dir)
        .unwrap_or(&file_path);

    // Extract job_id and filename from the relative path
    let mut path_parts = relative_path.iter();
    let job_id = path_parts.next().unwrap().to_string_lossy();
    let filename = path_parts.next().unwrap().to_string_lossy();

    let base_url = if config.external_url.is_empty() {
        format!(
            "{}://{}",
            match config.use_https {
                true => "https",
                false => "http",
            },
            config.address()
        )
    } else {
        config.external_url
    };

    let file_url = format!("{}/files/{}/{}", base_url, job_id, encode(&filename));

    Json(DownloadResponse {
        success: true,
        file_url: Some(file_url),
        error: None,
    })
}
