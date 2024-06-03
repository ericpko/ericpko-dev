use std::fs;

use axum::{extract::Path, response::IntoResponse};
use pulldown_cmark::{html, Parser};

use crate::templates;

pub async fn home() -> impl IntoResponse {
    templates::IndexTemplate
}

pub async fn projects() -> impl IntoResponse {
    templates::ProjectsTemplate
}

pub async fn blogs() -> impl IntoResponse {
    let blogs = list_blogs();
    templates::BlogsTemplate { posts: blogs }
}

pub async fn blog(Path(blog_id): Path<String>) -> impl IntoResponse {
    let (blog_id, html_content) = get_blog(blog_id);
    templates::BlogTemplate {
        title: blog_id,
        content: html_content,
    }
}

fn list_blogs() -> Vec<(String, String)> {
    let mut posts = Vec::new();

    let blogs_path = "content/blogs";
    let paths = fs::read_dir(blogs_path).expect("Failed to read blogs directory");

    // TODO: handle errors
    for path in paths {
        let path = path.unwrap().path();
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        posts.push((format!("/blogs/{}", filename), filename));
    }

    return posts;
}

fn get_blog(blog_id: String) -> (String, String) {
    let file_path = format!("content/blogs/{}.md", blog_id);
    let markdown_content = fs::read_to_string(file_path).expect("Failed to read file");

    // Configure the markdown parser
    let parser = Parser::new_ext(&markdown_content, pulldown_cmark::Options::all());

    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    (blog_id, html_content)
}
