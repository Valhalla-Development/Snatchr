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

    // Bind TCP listener to the configured address
    let listener = TcpListener::bind(&config.address()).await.unwrap();

    // Print tree-style startup banner
    println!();
    println!("🚀 SNATCHR v{}", env!("CARGO_PKG_VERSION"));
    println!("├── 📍 Server: http://{}", config.address());
    println!("├── ⚙️  Configuration:");

    // Log configuration values from the loaded config
    println!("│   ├── PORT = {}", config.port);
    println!("│   ├── HOST = {}", config.host);
    println!("│   ├── EXTERNAL_URL = {}", config.external_url);
    println!("│   ├── USE_HTTPS = {}", config.use_https);
    println!("│   ├── ENABLE_WEB_UI = {}", config.enable_web_ui);
    println!("│   ├── DOWNLOAD_DIR = {}", config.download_dir);
    println!(
        "│   ├── CLEANUP_AFTER_MINUTES = {}",
        config.cleanup_after_minutes
    );
    println!(
        "│   ├── MAX_CONCURRENT_DOWNLOADS = {}",
        config.max_concurrent_downloads
    );
    println!("│   ├── TIMEOUT_SECONDS = {}", config.timeout_seconds);
    println!("│   ├── VIDEO_QUALITY = {:?}", config.video_quality);
    println!("│   ├── VIDEO_CODEC = {:?}", config.video_codec);
    println!("│   ├── AUDIO_QUALITY = {:?}", config.audio_quality);
    println!("│   └── AUDIO_CODEC = {:?}", config.audio_codec);

    println!("├── 📋 Status:");
    if std::fs::read_to_string(".env").is_err() {
        println!(
            "│   └── ⚠️  No .env file found, using defaults - create one or copy .env.example"
        );
    } else {
        println!("│   └── ✅ Configuration loaded from .env");
    }
    println!("└── 🚀 Ready!");

    println!();

    // Start cleanup scheduler in background
    tokio::spawn(start_cleanup_scheduler());

    // Start serving requests
    axum::serve(listener, app).await.unwrap();
}
