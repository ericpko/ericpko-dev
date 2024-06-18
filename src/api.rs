use axum::{response::Html, routing::get, Router};

pub fn init_api_router() -> Router {
    Router::new().route("/hello", get(hello_from_the_server))
}

async fn hello_from_the_server() -> Html<&'static str> {
    Html(
        r#"
        Hello from the server! This is an example of htm<span class="text-blue-600">x</span>.
        "#,
    )
}
