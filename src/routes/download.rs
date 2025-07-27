use axum::Json;
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

pub async fn download_handler(payload: Json<DownloadRequest>) -> Json<DownloadResponse> {
    let job_id = Uuid::new_v4().to_string();
    
    start_background_download(payload.url.clone(), job_id.clone()).await;

    let res = Json(DownloadResponse { job_id });

    res
}