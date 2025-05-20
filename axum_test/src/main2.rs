use axum::{
    extract::{Path, Query, State},
    http::Uri,
    response::IntoResponse,
    routing::get,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
struct CreateUser {
    email: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        id: "Hello from state".to_string(),
    };

    let app = axum::Router::new()
        .route("/{id}/{name}", get(get_handler).post(post_handler))
        .route("/foo", get(handler))
        .route("/", get(q_handler))
        .with_state(state)
        .fallback(fallback);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on ...");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    id: String,
}

async fn get_handler(State(state): State<AppState>) -> String {
    String::from(state.id)
}
async fn post_handler(Path((id, name)): Path<(String, String)>) -> String {
    name
}
async fn q_handler(Query(q): Query<CreateUser>) -> String {
    format!("email: {}", q.email)
}
async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}

//------
use axum::{
    extract::FromRequestParts,
    http::{
        header::{HeaderValue, USER_AGENT},
        request::Parts,
        StatusCode,
    },
    Router,
};
use user_agent_parser::{Device, Engine, Product, UserAgentParser, CPU, OS};

struct ExtractUserAgent(HeaderValue);

impl<S> FromRequestParts<S> for ExtractUserAgent
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_agent) = parts.headers.get(USER_AGENT) {
            Ok(ExtractUserAgent(user_agent.clone()))
        } else {
            Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
        }
    }
}

async fn handler(ExtractUserAgent(user_agent): ExtractUserAgent) {
    let ua_parser = UserAgentParser::from_path("./regexes.yaml").unwrap();
    println!(
        "user_agent.to_str().unwrap(): {:?}",
        &user_agent.to_str().unwrap()
    );
    let ua = ua_parser.parse_device(user_agent.to_str().unwrap());

    println!("OS: {:?}", ua);
    // println!("Product: {:?}", ua.product);
    // println!("Device: {:?}", ua.device);
    // println!("CPU: {:?}", ua.cpu);
    // println!("Engine: {:?}", ua.engine);
}
