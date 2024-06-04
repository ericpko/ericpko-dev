use std::{
    fs::{self, File},
    io::{self, BufRead},
};

use axum::{extract::Path, response::IntoResponse};
use chrono::NaiveDate;
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

// fn list_blogs() -> Vec<(String, String, String, String)> {
//     let mut posts = Vec::new();
//     let blogs_path = "content/blogs";
//     let paths = std::fs::read_dir(blogs_path).expect("Failed to read blogs directory");

//     for path in paths {
//         let path = path.unwrap().path();
//         let filename = path.file_stem().unwrap().to_str().unwrap().to_string();

//         let file = File::open(&path).expect("Failed to open file");
//         let reader = io::BufReader::new(file);
//         let mut lines = reader.lines();

//         // Skip the first line (the opening HTML comment)
//         lines.next();

//         // Read the second line (date)
//         let date_str = lines.next().unwrap().unwrap().trim().to_string();

//         // Read the third line (title)
//         let title = lines.next().unwrap().unwrap().trim().to_string();

//         // Convert the date string to a NaiveDate
//         let date = NaiveDate::parse_from_str(&date_str, "%b %d, %Y").expect("Failed to parse date");

//         // Generate a short description (first few lines from the content after the header)
//         // Keep it simple for now
//         let description = "Short description goes here..."; // Placeholder for now

//         // Push formatted post directly
//         posts.push((
//             format!("/blogs/{}", filename),
//             date.format("%b %d, %Y").to_string(),
//             title,
//             description.to_string(),
//         ));
//     }

//     // Sort blogs by date (most recent first)
//     posts.sort_by(|a, b| b.1.cmp(&a.1));

//     posts
// }

fn list_blogs() -> Vec<(String, String, String, String, String)> {
    let mut posts = Vec::new();
    let blogs_path = "content/blogs";
    let paths = std::fs::read_dir(blogs_path).expect("Failed to read blogs directory");

    for path in paths {
        let path = path.unwrap().path();
        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();

        let file = File::open(&path).expect("Failed to open file");
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();

        // Skip the first line (the opening HTML comment)
        lines.next();

        // Read the second line (date)
        let date_str = lines.next().unwrap().unwrap().trim().to_string();

        // Read the third line (subject)
        let subject = lines.next().unwrap().unwrap().trim().to_string();

        // Read the fourth line (title)
        let title = lines.next().unwrap().unwrap().trim().to_string();

        // Read the following lines until closing HTML comment for the description
        let mut description = String::new();
        for line in lines {
            let line = line.unwrap();
            if line.starts_with("-->") {
                break;
            }
            description.push_str(&line);
            description.push_str("\n");
        }
        let description = description.trim().to_string();

        // Convert the date string to a NaiveDate
        let date = NaiveDate::parse_from_str(&date_str, "%b %d, %Y").expect("Failed to parse date");

        // Push formatted post directly
        posts.push((
            format!("/blogs/{}", filename),
            date.format("%b %d, %Y").to_string(),
            subject,
            title,
            description,
        ));
    }

    // Sort blogs by date (most recent first)
    posts.sort_by(|a, b| b.1.cmp(&a.1));

    posts
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
