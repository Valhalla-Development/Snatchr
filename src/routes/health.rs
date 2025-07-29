use axum::{Json, http::StatusCode};
use serde_json::json;

/*
 * Health check endpoint.
 * Returns a simple JSON response indicating the service is running.
 */
pub async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "service": "snatchr"
        })),
    )
}
