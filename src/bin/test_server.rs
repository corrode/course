// Basic server test to isolate compilation issues
use anyhow::Result;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", get(|| async { "Hello World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("🚀 Test server running on http://localhost:3001");

    axum::serve(listener, app).await?;
    Ok(())
}