use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{handlers, state, templates};

pub async fn home() -> impl IntoResponse {
    templates::IndexTemplate
}

pub async fn projects() -> impl IntoResponse {
    templates::ProjectsTemplate
}

pub async fn blogs(State(state): State<state::AppState>) -> impl IntoResponse {
    let blogs = state.blog_posts.read().await.clone();
    templates::BlogsTemplate { posts: blogs }
}

pub async fn blog(Path(blog_id): Path<String>) -> impl IntoResponse {
    let (blog_id, html_content) = handlers::blog::get_blog(blog_id);
    templates::BlogTemplate {
        title: blog_id,
        content: html_content,
    }
}
