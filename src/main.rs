use axum::{
    routing::any,
    Router,
};
use reqwest::Client;
use std::net::SocketAddr;

use once_cell::sync::Lazy;
use tracing::info;
use tracing_subscriber::EnvFilter;
use dotenv::dotenv;
use std::env;
use clap::Parser;

use handler::proxy_handler;
use command_args::Args;

use crate::crypto::generate_keys;

mod handler;
mod crypto;
mod director;
mod command_args;

// HMAC_KEY value from .env file..
static HMAC_KEY: Lazy<String> = Lazy::new(|| {
    env::var("HMAC_KEY")
        .expect("HMAC_KEY must be set in `.env` file")
});

// SIG_KEY value from .env file..
static SIG_KEY: Lazy<String> = Lazy::new(|| {
    env::var("SIG_KEY")
        .expect("SIG_KEY must be set in `.env` file")
});

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.generate_keys {
        generate_keys();
        return; 
    }
    if args.debug {
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("debug"))
            .with_target(true)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }

    // read .env file..
    dotenv()
        .expect("something goes wrong with `.env` file. maybe, you should create it");

    let client = Client::new(); // for NestJS

    let app = Router::new()
        .route("/{*path}", any(proxy_handler)) // Catch every route
        .with_state(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));

    println!("\t󰞀  Proxy Security Guard running on {},\n\t  with{} Debugging Outputs", 
        addr, 
        if args.debug {""} else {"out"}
    );

    info!("running on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
