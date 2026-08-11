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
use utils::logger;

pub use utils::logger::init as init_logging;

/*
 * Starts the Axum web server asynchronously.
 * Sets up routes and listens on the configured address.
 */
pub async fn run_server() {
    // Load configuration from environment variables
    let config = Config::from_env();

    // Build the application router with routes and request logging
    let app = Router::new()
        .route("/", get(download_page)) // GET / -> download_page (HTML interface)
        .route("/health", get(health_check)) // GET /health -> health_check
        .route("/download", post(download_handler)) // POST /download -> download_handler
        .route("/files/{video_id}/{filename}", get(serve_file)) // GET /files/:video_id/:filename -> serve_file
        .layer(axum::middleware::from_fn(logger::log_requests));

    // Bind TCP listener to the configured address
    let listener = TcpListener::bind(&config.address()).await.unwrap();

    // Print styled startup banner with the resolved configuration
    logger::print_banner(&config);

    // Start cleanup scheduler in background
    tokio::spawn(start_cleanup_scheduler());

    // Start serving requests
    axum::serve(listener, app).await.unwrap();
}
