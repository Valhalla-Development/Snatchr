use axum::{extract::Path, http::StatusCode, response::Response};
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tracing::{error, warn};

use crate::config::Config;

/// Serves downloaded files: /files/{job_id}/{filename}
pub async fn serve_file(
    Path((job_id, filename)): Path<(String, String)>,
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

    // Open and stream the file
    let file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open file {}: {}", file_path.display(), e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let stream = ReaderStream::new(file);
    let body = axum::body::Body::from_stream(stream);

    Ok(Response::builder()
        .header("content-type", "application/octet-stream")
        .header(
            "content-disposition",
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(body)
        .unwrap())
}
