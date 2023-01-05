use axum::{routing::get, Router};
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);

    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing_subscriber::fmt::init();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    tracing::debug!("listening on {}", addr);
}

async fn root() -> &'static str {
    "Hello, world!"
}
