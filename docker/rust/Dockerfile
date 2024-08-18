FROM rust:1.72

WORKDIR /backend

# Copy the contents of your Rust project to the container
COPY ./backend .

WORKDIR /backend

# Build the Rust project
RUN cargo build --release
