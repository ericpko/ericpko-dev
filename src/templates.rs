use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "blogs.html")]
pub struct BlogsTemplate {
    pub posts: Vec<(String, String)>, // (url, title) tuples
}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {
    pub title: String,
    pub content: String,
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate;
