use axum::response::IntoResponse;
use axum::http::StatusCode;

use serde_json::{json, Value};

pub async fn ping_response() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn health_status() -> axum::extract::Json<Value> {
    json!({"Server Status":"Healthy"}).into()
}