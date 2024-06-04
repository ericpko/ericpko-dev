use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "blogs.html")]
pub struct BlogsTemplate {
    // (url, date, subject, title, description) tuples
    pub posts: Vec<(String, String, String, String, String)>,
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
