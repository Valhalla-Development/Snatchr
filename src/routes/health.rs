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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn reports_healthy_service() {
        let (status, Json(body)) = health_check().await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["status"], "healthy");
        assert_eq!(body["service"], "snatchr");
    }
}
