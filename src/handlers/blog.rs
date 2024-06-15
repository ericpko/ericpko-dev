use std::{
    fs::{self, File},
    io::{self, BufRead},
};

use chrono::NaiveDate;
use pulldown_cmark::{html, Parser};

pub fn list_blogs() -> Vec<(String, String, String, String, String)> {
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
    posts.sort_by(|a, b| {
        let date_a = NaiveDate::parse_from_str(&a.1, "%b %d, %Y").unwrap();
        let date_b = NaiveDate::parse_from_str(&b.1, "%b %d, %Y").unwrap();
        date_b.cmp(&date_a)
    });

    posts
}

pub fn get_blog(blog_id: String) -> (String, String) {
    let file_path = format!("content/blogs/{}.md", blog_id);
    let markdown_content = fs::read_to_string(&file_path).expect("Failed to read file");

    // Extract the title from the comment
    let file = File::open(&file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    // Skip the first line (the opening HTML comment)
    lines.next();

    // Read the second line (date)
    lines.next();

    // Read the third line (subject)
    lines.next();

    // Read the fourth line (title)
    let title = lines.next().unwrap().unwrap().trim().to_string();

    // Configure the markdown parser
    let parser = Parser::new_ext(&markdown_content, pulldown_cmark::Options::all());

    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    (title, html_content)
}
