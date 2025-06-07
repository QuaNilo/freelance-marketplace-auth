mod config;
mod routes;
mod db;
mod utils;
mod models;
mod traits;

use std::net::SocketAddr;
use axum::Server;
use crate::config::{Settings};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    let settings = Settings::new();

    let app = routes::router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 45000));
    println!("Listening on {}", addr);
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
