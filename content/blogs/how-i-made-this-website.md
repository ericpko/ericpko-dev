<!---
Jun 18, 2024
Rust/htmx
Rust, htmx, and Tailwindcss
Rust, htmx, and tailwindcss. Keep it simple, stupid.
-->


## Every few years...
I get the itch to redesign my website. I've tried a lot of
different frameworks over the years, but I've never quite been happy with the
results. This time, I decided to keep it simple.

<br>

There are a few things I wanted to accomplish with this redesign:
- It has to be simple, only necessary dependencies.
- I want to write blogs in markdown.
- I want to have complete control over the code.

## In the past...
I've used frameworks like Sveltekit, Next.js, Static Site
Generators, Astro, and others. Astro probably makes the most sense for my
goals, and would be a good option here, but for whatever reason, I just
don't like when I don't understand how something works under the hood.
I know I can write markdown in any of those frameworks, but I like
having the control over how it's done. It also gives me insight as to how those
frameworks might implement it.

# Rust, htmx, and tailwindcss
As web frameworks keep evolving and becoming more complex, there's
something very appealing about going back to a very basic web
formula:
- A backend web framework for handling requests. I chose Axum.
- HTML and CSS for the frontend. I used Askama for the HTML templating
  and Tailwindcss for the CSS.
- htmx for the dynamic parts (not that there are any dynamic
  parts on my simple website, but what-the-heck, I wanted to try it).
- Shuttle.rs for deployment.

Recently, I came across an [article](https://joeymckenzie.tech/blog/templates-with-rust-axum-htmx-askama)
by [Joey McKenzie](https://joeymckenzie.tech) on how he used
this formula in action. Naturally, as I was trying out his tech stack,
I couldn't help but notice how clean his website was and decided to take
inspiration from it (I basically just copied it because it's awesome).

<br>

I won't go into the step-by-step details on how I made this site
because you can find that in
the article above, or on the
[shuttle.rs](https://www.shuttle.rs/blog/2023/10/25/htmx-with-rust) blog.
However, I will describe how I made the blog part of the website.

## Blogging
I wanted to write blogs in markdown, so I needed a way to convert markdown
to HTML. I could have used Askama for this, but I wanted to keep the
markdown separate from the HTML. I also wanted to keep the markdown files
in a separate directory from the rest of the code. I decided to use
[pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark),
a markdown parser written in Rust. I wrote a simple function that reads
the markdown files from a directory and converts them to HTML. I then
used Askama to render the HTML.

### Directory structure

```
project
│
└───content
│   │
│   └───blogs
│       │   blog1.md
│       │   blog2.md
│       │   ...
│
└───assets
│   │   blog.css
│   │   ...
│
└───...
│
│   Cargo.toml
│   package.toml
│   ...
```

### Reading markdown files

I have a simple function that reads the markdown files from the
`content/blogs`.

```rust
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
```

<br>

Then to read the actual markdown content, I have another function.

```rust
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
```
