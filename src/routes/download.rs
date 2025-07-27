use axum::{Json, response::Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Config;
use crate::handlers::downloader::download_video;

#[derive(Deserialize)]
pub struct DownloadRequest {
    url: String,
}

#[derive(Serialize)]
pub struct DownloadResponse {
    file_url: String,
}

#[axum::debug_handler]
pub async fn download_handler(
    Json(payload): Json<DownloadRequest>,
) -> Result<Json<DownloadResponse>> {
    let job_id = Uuid::new_v4().to_string();
    let url = payload.url.clone();

    let (file_path, _duration) =
        tokio::task::spawn_blocking(move || download_video(url, job_id).map_err(|e| e.to_string()))
            .await
            .map_err(|e| format!("Task join error: {}", e))?
            .map_err(|e| format!("Download error: {}", e))?;

    let config = Config::from_env();

    let file_url = format!(
        "/{}",
        file_path
            .strip_prefix(&config.download_dir)
            .unwrap_or(&file_path)
            .to_string_lossy()
    );

    Ok(Json(DownloadResponse { file_url }))
}
