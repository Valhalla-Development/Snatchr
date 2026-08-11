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

fn is_valid_video_url(url: &str) -> bool {
    let Ok(uri) = url.parse::<axum::http::Uri>() else {
        return false;
    };

    matches!(uri.scheme_str(), Some("http" | "https")) && uri.host().is_some()
}

#[axum::debug_handler]
pub async fn download_handler(Json(payload): Json<DownloadRequest>) -> Json<DownloadResponse> {
    let job_id = Uuid::new_v4().to_string();
    let url = payload.url.clone();
    let config = Config::from_env();

    // Validate the URL shape locally. yt-dlp performs definitive extractor validation.
    if !is_valid_video_url(&url) {
        return Json(DownloadResponse {
            success: false,
            file_url: None,
            error: Some("Unsupported or invalid video URL".to_string()),
        });
    }

    // Run the download_video function on a blocking thread since it performs sync operations
    let job_id_clone = job_id.clone();
    let result = timeout(
        Duration::from_secs(config.timeout_seconds),
        tokio::task::spawn_blocking(move || download_video(url, job_id).map_err(|e| e.to_string())),
    )
    .await;

    let (file_path, _duration) = match result {
        Ok(task_result) => match task_result {
            Ok(download_result) => match download_result {
                Ok(result) => result,
                Err(e) => {
                    // The downloader already error-logged the failure with full context
                    return Json(DownloadResponse {
                        success: false,
                        file_url: None,
                        error: Some(format!("Download error: {}", e)),
                    });
                }
            },
            Err(e) => {
                error!(job = %job_id_clone, error = %e, "Worker task panicked");
                return Json(DownloadResponse {
                    success: false,
                    file_url: None,
                    error: Some(format!("Task join error: {}", e)),
                });
            }
        },
        Err(_) => {
            error!(job = %job_id_clone, timeout_s = config.timeout_seconds, "Download timed out");
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

    // Extract video_id and filename from the relative path
    let mut path_parts = relative_path.iter();
    let video_id = path_parts.next().unwrap().to_string_lossy();
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

    let file_url = format!("{}/files/{}/{}", base_url, video_id, encode(&filename));

    Json(DownloadResponse {
        success: true,
        file_url: Some(file_url),
        error: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn rejects_invalid_url_without_starting_download() {
        let Json(response) = download_handler(Json(DownloadRequest {
            url: "not a URL".to_string(),
        }))
        .await;

        assert!(!response.success);
        assert!(response.file_url.is_none());
        assert_eq!(
            response.error.as_deref(),
            Some("Unsupported or invalid video URL")
        );
    }

    #[test]
    fn response_omits_empty_optional_fields() {
        let response = DownloadResponse {
            success: true,
            file_url: Some("http://localhost/files/id/video.mp4".to_string()),
            error: None,
        };

        let json = serde_json::to_value(response).expect("response should serialize");

        assert_eq!(json["success"], true);
        assert_eq!(json["file_url"], "http://localhost/files/id/video.mp4");
        assert!(json.get("error").is_none());
    }
}
