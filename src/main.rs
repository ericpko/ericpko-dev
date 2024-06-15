mod api;
mod errors;
mod handlers;
mod models;
mod router;
mod routes;
mod state;
mod templates;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let blog_posts = handlers::blog::list_blogs(); // Read blog posts

    let state = state::AppState::new(blog_posts);

    let router = router::init_router(state.clone());

    Ok(router.into())
}
