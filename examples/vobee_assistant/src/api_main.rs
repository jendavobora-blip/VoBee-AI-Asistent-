mod chatbot;
mod response_patterns;
mod api_server;

use api_server::start_server;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    println!("🚀 Starting VoBee API Server...");
    
    if let Err(e) = start_server(port).await {
        eprintln!("❌ Server error: {}", e);
        std::process::exit(1);
    }
}
