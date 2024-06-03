use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use crate::{api, routes};

pub fn init_router() -> Router {
    let api_router = api::init_api_router();
    Router::new()
        .nest("/api", api_router)
        .route("/", get(routes::home))
        .route("/blogs", get(routes::blogs))
        .route("/blogs/:blog_id", get(routes::blog))
        .route("/projects", get(routes::projects))
        // .route("/todos", get(routes::fetch_todos).post(routes::create_todo))
        // Add a tower service route to serve everything under the assets/ folder
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(LiveReloadLayer::new())
}
