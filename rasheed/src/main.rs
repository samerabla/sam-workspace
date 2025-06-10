use axum::response::Html;
use axum::{Router, routing::get};
use rand::Rng;
use rand::prelude::*;
use std::io::BufRead;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server running on port 3001");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    let file = std::fs::File::open("./src/quotes.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let quotes = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let random = rand::rng().random_range(0..quotes.len());
    quotes[random].clone()
}
async fn hello() -> Html<String> {
    Html(include_str!("../rash.html").to_string())
}
