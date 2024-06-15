use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    // Using RwLock to allow concurrent read access
    pub blog_posts: Arc<RwLock<Vec<(String, String, String, String, String)>>>,
}

impl AppState {
    pub fn new(blog_posts: Vec<(String, String, String, String, String)>) -> Self {
        AppState {
            blog_posts: Arc::new(RwLock::new(blog_posts)),
        }
    }
}
