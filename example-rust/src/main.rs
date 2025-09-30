use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new()
        .route("/", get(hello_world));

    // Run the server on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running on http://localhost:3000");
    
    axum::serve(listener, app)
        .await
        .unwrap();
}

// Handler function that returns "Hello, World!"
async fn hello_world() -> &'static str {
    "Hello, World!"
}