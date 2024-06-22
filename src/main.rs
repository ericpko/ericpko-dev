mod api;
mod errors;
mod handlers;
mod router;
mod routes;
mod state;
mod templates;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Read blog posts at startup and save them in the app state
    let blog_posts = handlers::blog::list_blogs();
    let state = state::AppState::new(blog_posts);

    // initialize our app with some routes
    let app = router::init_router(state.clone()); // app/router
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
