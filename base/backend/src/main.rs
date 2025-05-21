#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use frontend::App;

use sam_error::SamError;
use std::{any::Any, sync::Arc};

use axum::{
    body::{self, Body, Bytes},
    extract::{FromRequest, Request},
    http::{HeaderValue, StatusCode, Uri},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
    Method,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::{catch_panic::CatchPanicLayer, services::ServeDir};

use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use error::{error_middleware, handle_error};
use user::{auth_middleware, user_routes, Claims, User, UserInfo};

mod error;
mod response;
mod user;

#[tokio::main]
async fn main() -> Result<(), SamError> {
    dotenvy::dotenv().ok();
    init_tracing();

    let db_url = dotenvy::var("DATABASE_URL")
        .map_err(|_| SamError::MissingEnviromentVariable("DATABASE_URL".to_string()))?;
    //let db_url = "postgres://postgres:555@localhost:5432/test1".to_string();
    let pool = PgPool::connect(&db_url).await?;

    // Create the application state
    let state: AppState = AppState {
        pool: Arc::new(pool),
    };

    // Handle cors issues
    //-------------------

    // We add several origins for local host
    let allowed_origins = vec![
        HeaderValue::from_static("http://localhost:9000"),
        HeaderValue::from_static("http://127.0.0.1:9000"),
    ];

    let cors_layer = CorsLayer::new()
        // .allow_origin("http://127.0.0.1:9000".parse::<HeaderValue>().unwrap())
        .allow_origin(AllowOrigin::predicate(move |origin, _| {
            allowed_origins.contains(origin)
        }))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([ORIGIN, CONTENT_TYPE, ACCEPT, AUTHORIZATION])
        .allow_credentials(true)
        .expose_headers([http::header::SET_COOKIE]);

    let app = Router::new()
        .route("/", get(app_endpoint))
        .nest_service("/assets", ServeDir::new("assets"))
        .route(
            "/foo",
            get(handler_1).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route("/internal-error", get(internal_err_handler))
        .merge(user_routes(state.clone()))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            error_middleware,
        ))
        .layer(CatchPanicLayer::custom({
            let pool = state.pool.clone();
            move |error: Box<dyn Any + Send>| {
                let pool = pool.clone();
                tokio::task::spawn_blocking(move || {
                    tokio::runtime::Handle::current().block_on(async {
                        let sam_error = if let Some(panic_msg) = error.downcast_ref::<&str>() {
                            sam_error::any_with_log!(panic_msg.to_string())
                        } else {
                            sam_error::any_with_log!("Panic!!!!".to_string())
                        };
                        handle_error(Box::new(sam_error), &pool).await;
                    });
                });
                Response::new("An error occurred".to_string())
            }
        }))
        .layer(cors_layer)
        .with_state(state);
    //.fallback(fallback_handler)

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000 ...");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn app_endpoint() -> Html<String> {
    let mut app = VirtualDom::new(App);
    // rebuild the VirtualDom before rendering
    app.rebuild_in_place();

    let mut renderer = dioxus_ssr::Renderer::new();
    renderer.pre_render = true;

    // render the VirtualDom to HTML
    Html(renderer.render(&app))
    // Html(dioxus_ssr::render(&app))
}

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    n: i32,
}

struct ValidatedBody(Bytes);

impl<S> FromRequest<S> for ValidatedBody
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        // Perform validation (e.g., check if the body is not empty)
        if body.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Request body cannot be empty").into_response());
        }

        Ok(Self(body))
    }
}

async fn handler_1(request: Request) -> String {
    //let info = request.extensions().get::<Arc<UserInfo>>();
    // let body: Bytes = body::to_bytes(request.into_body(), usize::MAX)
    //     .await
    //     .unwrap();
    format!("successooo....")
    //format!("b: {:?} ", info)
}

async fn internal_err_handler() -> String {
    "Internal Erro".to_string()
}

async fn fallback_handler(request: Request) -> Response {
    println!("No route found for: {} {}", request.method(), request.uri());
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not Found".into())
        .unwrap()
}

async fn handler(ValidatedBody(body): ValidatedBody) -> Result<Json<MyData>, Response> {
    // Parse the body as JSON
    let data: MyData = serde_json::from_slice(&body).map_err(|err| {
        // If parsing fails, return a 400 Bad Request with the error message
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to parse JSON: {}", err),
        )
            .into_response()
    })?;

    // Return the parsed data as JSON
    Ok(Json(data))
}

#[derive(Clone, Debug)]
struct AppState {
    pool: Arc<PgPool>,
}

fn init_tracing() {
    let fmt_layer = fmt::layer().without_time();

    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(ErrorLayer::default()) // Enable automatic error logging
        .with(EnvFilter::from_default_env()); // Allow filtering with RUST_LOG

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}
