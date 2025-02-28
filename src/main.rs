use bang_search::{routes, config};
mod models;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Load configuration from config.yml
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yml".to_string());
    let config = match config::load_config(&config_path).await {
        Ok(cfg) => {
            println!("Loaded configuration from {}", config_path);
            cfg
        },
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            eprintln!("Using default configuration");
            config::default_config()
        }
    };
    
    // Build our application with routes
    let app = routes::create_router().with_state(config);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 9876));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
