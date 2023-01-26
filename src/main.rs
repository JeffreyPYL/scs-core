mod api;

use axum::routing::get;
use axum::{ Router, Server };

use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use api::{ ping_response, health_status };

#[tokio::main]
async fn main(){
    tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .init();

    let app = Router::new()
    .route("/", get(ping_response))
    .route("/health", get(health_status));

    // Run our application as a hyper server on http://localhost:3000.
    Server::bind(&"c".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}


pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri)
    )
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}