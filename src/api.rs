use axum::{routing::get, Router};

pub fn init_api_router() -> Router {
    Router::new().route("/hello", get(hello_from_the_server))
}

async fn hello_from_the_server() -> &'static str {
    "Hello from the server!"
}
