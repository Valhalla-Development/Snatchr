use axum::{Json, response::Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::handlers::downloader::start_background_download;

#[derive(Deserialize)]
pub struct DownloadRequest {
    url: String,
}

#[derive(Serialize)]
pub struct DownloadResponse {
    job_id: String,
}

pub async fn download_handler(
    Json(payload): Json<DownloadRequest>,
) -> Result<Json<DownloadResponse>> {
    let job_id = Uuid::new_v4().to_string();

    let url = payload.url.clone();
    let job_id_clone = job_id.clone();

    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(start_background_download(url, job_id_clone));
    });

    Ok(Json(DownloadResponse { job_id }))
}
