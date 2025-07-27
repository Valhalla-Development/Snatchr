use axum::{response::Html, routing::{get, post}, Router};
use tokio::net::TcpListener;

mod config;
use config::Config;

mod routes;
use routes::download::download_handler;

pub async fn run_server() {
    let config = Config::new();
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/download", post(download_handler));

    let listener = TcpListener::bind(&config.address()).await.unwrap();

    println!("Server is running on http://{}", config.address());
    
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Html<&'static str> {
    Html("Hello, world!")
}