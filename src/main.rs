/*
 * Entry point of the application.
 * Uses Tokio's async runtime to start the program asynchronously.
 *
 * Initializes the custom console logger (compact colorized format,
 * RUST_LOG-aware), then starts the server.
 */
#[tokio::main]
async fn main() {
    snatchr::init_logging();

    snatchr::run_server().await;
}
