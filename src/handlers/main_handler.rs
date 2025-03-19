use crate::AppState;
use axum::{Router, response::IntoResponse, routing::get};

pub fn main_router() -> Router<AppState> {
    Router::new().route("/hello", get(hello))
}

pub async fn hello() -> impl IntoResponse {
    "hello there from handler"
}
