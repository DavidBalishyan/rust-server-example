use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use std::{net::SocketAddr, path::PathBuf};

/// Example API handler
async fn hello_api() -> &'static str {
    "WTF!"
}

#[tokio::main]
async fn main() {
    // Path to built frontend from Rsbuild
    let static_dir = PathBuf::from("./dist");

    // Serve static files, fallback to index.html for SPA routing
    let serve_dir = ServeDir::new(static_dir.clone())
        .not_found_service(ServeDir::new(static_dir.join("index.html")));

    // API routes
    let api_routes = Router::new()
        .route("/data", get(hello_api));

    // Combine API + static file server
    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(serve_dir);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    // Axum 0.7 way — use tokio::net::TcpListener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
