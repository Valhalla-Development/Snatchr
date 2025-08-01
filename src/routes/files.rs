use axum::{
    extract::{Path, Query},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::path::PathBuf;
use tower::util::ServiceExt;
use tower_http::services::ServeFile;
use tracing::warn;

use crate::config::Config;

/// Serves downloaded files: /files/{job_id}/{filename}
/// Add ?stream=true to stream video instead of downloading
pub async fn serve_file(
    Path((job_id, filename)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, StatusCode> {
    let config = Config::from_env();

    // Construct path: download_dir/job_id/filename
    let file_path = PathBuf::from(&config.download_dir)
        .join(&job_id)
        .join(&filename);

    // Check if file exists and is actually a file
    if !file_path.exists() || !file_path.is_file() {
        warn!("File not found: {}", file_path.display());
        return Err(StatusCode::NOT_FOUND);
    }

    // Use tower-http's ServeFile to handle range requests automatically
    let request = axum::http::Request::builder()
        .body(axum::body::Body::empty())
        .unwrap();

    let mut response = ServeFile::new(&file_path)
        .oneshot(request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Default to download unless stream=true is specified
    if !params.get("stream").map_or(false, |v| v == "true") {
        response.headers_mut().insert(
            "content-disposition",
            HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
                .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
        );
    }

    Ok(response.into_response())
}
