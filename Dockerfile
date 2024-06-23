# Stage 1: Build the Rust backend
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Copy source code and build the Rust application
COPY . .
RUN cargo build --release --bin ericpko-dev

# Stage 2: Create the final runtime image
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Copy everything from the builder stage
COPY --from=builder /app/target/release/ericpko-dev /usr/local/bin/ericpko-dev
COPY --from=builder /app /app

# Expose necessary ports (if any, modify as needed)
EXPOSE 8080

# Define the entry point for the container
ENTRYPOINT ["/usr/local/bin/ericpko-dev"]



# # Stage 1: Build the Rust backend
# FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
# WORKDIR /app

# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM chef AS builder
# COPY --from=planner /app/recipe.json recipe.json
# # Build dependencies - this is the caching Docker layer!
# RUN cargo chef cook --release --recipe-path recipe.json
# # Copy source code and build the Rust application
# COPY . .
# RUN cargo build --release --bin ericpko-dev

# # Stage 2: Create the final runtime image
# FROM debian:bookworm-slim AS runtime
# WORKDIR /app
# # Copy everything from the builder stage
# COPY --from=builder /app /app

# # Expose necessary ports (if any, modify as needed)
# EXPOSE 8080

# # Define the entry point for the container
# ENTRYPOINT ["/app/target/release/ericpko-dev"]
