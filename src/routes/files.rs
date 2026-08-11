use axum::{
    extract::{Path, Query},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::path::PathBuf;
use tower::util::ServiceExt;
use tower_http::services::ServeFile;
use tracing::{debug, warn};

use crate::config::Config;

/// Serves downloaded files: /files/{video_id}/{filename}
/// Add ?stream=true to stream video instead of downloading
pub async fn serve_file(
    Path((video_id, filename)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, StatusCode> {
    let config = Config::from_env();

    // Construct path: download_dir/video_id/filename
    let file_path = PathBuf::from(&config.download_dir)
        .join(&video_id)
        .join(&filename);

    // Check if file exists and is actually a file
    if !file_path.exists() || !file_path.is_file() {
        warn!(path = %file_path.display(), "File not found");
        return Err(StatusCode::NOT_FOUND);
    }

    // Update access marker for smart caching
    let is_stream = params.get("stream").is_some_and(|v| v == "true");

    if !is_stream {
        let access_marker = file_path.parent().unwrap().join(".last_accessed");

        if let Err(e) = std::fs::write(&access_marker, "") {
            warn!(video = %video_id, error = %e, "Failed to update access marker");
        } else {
            debug!(video = %video_id, "Access marker updated");
        }
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
    if !is_stream {
        response.headers_mut().insert(
            "content-disposition",
            HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
                .unwrap_or_else(|_| HeaderValue::from_static("attachment")),
        );
    }

    Ok(response.into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn returns_not_found_for_missing_file() {
        let result = serve_file(
            Path((Uuid::new_v4().to_string(), "missing.mp4".to_string())),
            Query(HashMap::new()),
        )
        .await;

        assert!(matches!(result, Err(StatusCode::NOT_FOUND)));
    }
}
