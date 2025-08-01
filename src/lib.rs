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
use routes::page::download_page;

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
        .route("/", get(download_page)) // GET / -> download_page (HTML interface)
        .route("/health", get(health_check)) // GET /health -> health_check
        .route("/download", post(download_handler)) // POST /download -> download_handler
        .route("/files/{job_id}/{filename}", get(serve_file)); // GET /files/:job_id/:filename -> serve_file

    // Print web UI status
    if config.enable_web_ui {
        println!("Web UI enabled at http://{}", config.address());
    } else {
        println!(
            "Web UI disabled - API only mode at http://{}",
            config.address()
        );
    }

    // Bind TCP listener to the configured address
    let listener = TcpListener::bind(&config.address()).await.unwrap();

    // Print server start info
    println!("Server is running on http://{}", config.address());
    if !config.external_url.is_empty() {
        println!("External URL: {}", config.external_url);
    }

    // Start cleanup scheduler in background
    tokio::spawn(start_cleanup_scheduler());

    // Start serving requests
    axum::serve(listener, app).await.unwrap();
}
