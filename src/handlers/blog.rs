use std::{
    fs::{self, File},
    io::{self, BufRead},
};

use chrono::NaiveDate;
use comrak::{markdown_to_html, Options};

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
            description.push('\n');
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

    // Convert markdown to html
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.description_lists = true;
    options.extension.multiline_block_quotes = true;
    options.extension.math_dollars = true;
    options.extension.math_code = true;
    let html = markdown_to_html(&markdown_content, &options);

    (title, html)
}
