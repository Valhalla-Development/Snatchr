use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

mod config;
use config::Config;

mod routes;
use routes::download::download_handler;
use routes::files::serve_file;
use routes::health::health_check;

mod handlers;
mod utils;
use utils::cleanup::start_cleanup_scheduler;

/*
 * Starts the Axum web server asynchronously.
 * Sets up routes and listens on the configured address.
 */
pub async fn run_server() {
    // Load configuration from environment variables
    let config = Config::from_env();

    // Build the application router with routes
    let app = Router::new()
        .route("/health", get(health_check)) // GET /health -> health_check
        .route("/download", post(download_handler)) // POST /download -> download_handler
        .route("/files/{job_id}/{filename}", get(serve_file)); // GET /files/:job_id/:filename -> serve_file

    // Bind TCP listener to the configured address
    let listener = TcpListener::bind(&config.address()).await.unwrap();

    // Print server start info
    println!("Server is running on http://{}", config.address());

    // Start cleanup scheduler in background
    tokio::spawn(start_cleanup_scheduler());

    // Start serving requests
    axum::serve(listener, app).await.unwrap();
}
