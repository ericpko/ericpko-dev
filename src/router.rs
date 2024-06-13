use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use crate::{api, routes, AppState};

pub fn init_router(state: AppState) -> Router {
    let router = Router::new()
        .route("/", get(routes::home))
        .route("/blogs", get(routes::blogs))
        .route("/blogs/:blog_id", get(routes::blog))
        .route("/projects", get(routes::projects))
        // Add a tower service route to serve everything under the assets/ folder
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(LiveReloadLayer::new())
        .with_state(state);

    let api_router = api::init_api_router();
    router.nest("/api", api_router)
}
