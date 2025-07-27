/*
 * Entry point of the application.
 * Uses Tokio's async runtime to start the program asynchronously.
 *
 * Sets up the tracing subscriber for structured logging:
 * - Logs messages at INFO level and above.
 *
 * Finally, it calls and awaits the main server run function from the `snatchr` module.
 */
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    snatchr::run_server().await;
}
