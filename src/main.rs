use std::sync::Arc;
use tokio::sync::RwLock;

mod api;
mod errors;
mod models;
mod router;
mod routes;
mod templates;

#[derive(Clone)]
struct AppState {
    // Using RwLock to allow concurrent read access
    blog_posts: Arc<RwLock<Vec<(String, String, String, String, String)>>>,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = router::init_router();

    Ok(router.into())
}
