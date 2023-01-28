pub mod config;

use axum::{response::Html, routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
