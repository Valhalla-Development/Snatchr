use axum::{
    Router,
    response::Html,
    routing::{get, post},
};
use tokio::net::TcpListener;

mod config;
use config::Config;

mod routes;
use routes::download::download_handler;

mod handlers;

pub async fn run_server() {
    let config = Config::from_env();
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
