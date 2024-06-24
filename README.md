# Personal dev site

[https://ericpko.dev](https://ericpko.dev)

My personal developer website. Built using Rust, htmx, tailwindcss, and hosted on fly.io.

## Usage

Run the app and go to <http://localhost:8080>

```
cargo build
bun i

bunx tailwindcss -i styles/tailwind.css -o assets/main.css --watch
// or
bun tw-watch

cargo watch -cx run
```
