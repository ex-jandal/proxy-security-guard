use axum::{
    routing::any,
    Router,
};
use reqwest::Client;
use std::net::SocketAddr;

use handler::proxy_handler;
mod handler;
mod verification;
mod director;

use once_cell::sync::Lazy;
use tracing::info;
use dotenv::dotenv;
use std::env;

/// SIG_KEY value from .env file..
static SIG_KEY: Lazy<String> = Lazy::new(|| {
    env::var("SIG_KEY")
        .expect("SIG_KEY must be set in `.env` file")
});

/// deprecated feature
const DBG_MODE: bool = false;

#[tokio::main]
async fn main() {
    // read .env file..
    dotenv()
        .expect("something goes wrong with `.env` file. maybe, you should create it");

    let client = Client::new(); // for NestJS

    let app = Router::new()
        .route("/{*path}", any(proxy_handler)) // Catch every route
        .with_state(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("\n\t󰞀  Fanouni Security Guard running on {},\n\t  with{} Debugging Outputs", 
        addr, 
        if DBG_MODE {""} else {"out"}
    );
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

