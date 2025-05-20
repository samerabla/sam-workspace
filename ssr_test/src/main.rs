use dioxus::prelude::*; // backend/src/main.rs

use axum::{
    routing::{get, post},
    Json, Router,
};
use dioxus::LaunchBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[tokio::main]
async fn main() {
    // Create your backend-only routes (Axum style)
    let api_routes = Router::new().route("/login", post(login));

    // Launch Dioxus fullstack app with your Axum config
    LaunchBuilder::new(App)
        .addr(([127, 0, 0, 1], 3000))
        .routes(move |_| api_routes.clone()) // full control here
        .launch();
}

async fn login(Json(payload): Json<LoginRequest>) -> &'static str {
    if payload.email == "samerfamilie@gmail.com" && payload.password == "555" {
        set_cookie("token", "valid_token_here");
        "Login successful"
    } else {
        "Login failed"
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        div {
            h1 { "Hello from Dioxus Fullstack!" }
        }
    }
}
