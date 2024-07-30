use std::sync::Arc;
use tokio::sync::RwLock;

type BlogPostsType = Arc<RwLock<Vec<(String, String, String, String, String)>>>;

#[derive(Clone)]
pub struct AppState {
    // Using RwLock to allow concurrent read access
    pub blog_posts: BlogPostsType,
}

impl AppState {
    pub fn new(blog_posts: Vec<(String, String, String, String, String)>) -> Self {
        AppState {
            blog_posts: Arc::new(RwLock::new(blog_posts)),
        }
    }
}
