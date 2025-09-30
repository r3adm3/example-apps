# Rust Hello World Web App

A simple web server built with Rust that responds with "Hello, World!" when accessed through a browser.

## Prerequisites

### Installing Rust

If you don't have Rust installed, follow these steps:

**On macOS, Linux, or Unix-like OS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**On Windows:**
Download and run [rustup-init.exe](https://rustup.rs/) from the official Rust website.

After installation, restart your terminal and verify Rust is installed:
```bash
rustc --version
cargo --version
```

## What This Code Does

This application creates a basic HTTP web server using the Axum web framework. Here's what happens:

1. **Server Setup**: The application starts an asynchronous web server on `localhost:3000`
2. **Route Definition**: It defines a single route (`/`) that responds to GET requests
3. **Handler Function**: When someone visits the root URL, the `hello_world()` function returns the text "Hello, World!"
4. **Async Runtime**: Uses Tokio as the async runtime to handle requests efficiently

### Key Components

- **axum**: A modern, ergonomic web framework for Rust
- **tokio**: An asynchronous runtime that enables concurrent request handling
- **Router**: Defines the URL routes and their corresponding handlers

## How to Run

1. **Run the application:**
   ```bash
   cargo run
   ```

2. **Test it out:**
   - Open your web browser and go to `http://localhost:3000`
   - You should see "Hello, World!" displayed
   - Or use curl from another terminal:
     ```bash
     curl http://localhost:3000
     ```

3. **Stop the server:**
   - Press `Ctrl+C` in the terminal where the server is running

## Troubleshooting

**Port already in use**: If port 3000 is already occupied, you can change it in `main.rs`:
```rust
let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
```

**Build errors**: Make sure you have the latest stable Rust:
```bash
rustup update stable
```

## Running with Docker

If you prefer to run the application in a container, you can use Docker:

### Prerequisites
- Install [Docker](https://docs.docker.com/get-docker/) on your system

### Build the Docker Image

From the project root directory (where the Dockerfile is located):
```bash
docker build -t example-rust .
```

This will:
- Use a multi-stage build to compile your Rust application
- Create a minimal runtime image with just the compiled binary
- Tag the image as `example-rust`

### Run the Container

```bash
docker run -p 3000:3000 example-rust
```

The `-p 3000:3000` flag maps port 3000 from the container to port 3000 on your host machine.

### Run in Detached Mode

To run the container in the background:
```bash
docker run -d -p 3000:3000 --name hello-rust example-rust
```

### Stop the Container

```bash
docker stop hello-rust
docker rm hello-rust
```

### Optional: Create a .dockerignore File

Create a `.dockerignore` file to speed up Docker builds:
```
target/
.git/
.gitignore
*.md
```

## Next Steps

To expand this application, you could:
- Add more routes (e.g., `/about`, `/api/users`)
- Return JSON responses instead of plain text
- Add request logging and error handling
- Serve static files or HTML templates
- Connect to a database

Check out the [Axum documentation](https://docs.rs/axum/latest/axum/) for more examples and features.
