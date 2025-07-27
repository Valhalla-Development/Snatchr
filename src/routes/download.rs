use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DownloadRequest {
    url: String,
}

#[derive(Serialize)]
pub struct DownloadResponse {
    job_id: String,
}

pub async fn download_handler(payload: Json<DownloadRequest>) -> Json<DownloadResponse> {
    let job_id = Uuid::new_v4();

    let res = Json(DownloadResponse { job_id: job_id.to_string() });

    res
}