#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
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
use tower_http::cors::CorsLayer;
use tower_http::{catch_panic::CatchPanicLayer, services::ServeDir};

use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use error::{error_middleware, handle_error};
use user::{auth_middleware, user_routes, Claims, User, UserInfo};

use frontend::*;
/* Modulues */
mod error;
mod frontend;
mod response;
mod user;

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
    let info = request.extensions().get::<Arc<UserInfo>>();
    // let body: Bytes = body::to_bytes(request.into_body(), usize::MAX)
    //     .await
    //     .unwrap();
    // format!("successooo....")
    format!("b: {:?} ", info)
}

// async fn auth_middleware(request: Request, next: Next) -> Result<Response, Response> {
//     if let Some(auth) = request.headers().get("Authorization") {
//         if auth == "Bearer secret-token" {
//             return Ok(next.run(request).await);
//         }
//     }
//     Err((StatusCode::UNAUTHORIZED, "Not auth...").into_response())
// }

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
    // .with_file(true) // Include file name in logs
    // .with_line_number(true) // Include line number in logs
    // .with_target(true); // Show the module where the log came from

    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(ErrorLayer::default()) // Enable automatic error logging
        .with(EnvFilter::from_default_env()); // Allow filtering with RUST_LOG

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}

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

    // // Use this later after modifying the period in the fn
    // let pool = Arc::clone(&state.pool);
    // tokio::spawn(cleanup_unverified_users(pool));

    // let layer = CorsLayer::new()
    //     .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
    //     .allow_methods([Method::GET, Method::POST])
    //     .allow_headers([ORIGIN, CONTENT_TYPE, ACCEPT, AUTHORIZATION])
    //     .allow_credentials(true);

    // let layer = CorsLayer::permissive();

    let layer = CorsLayer::new()
        .allow_origin("http://127.0.0.1:9000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([ORIGIN, CONTENT_TYPE, ACCEPT, AUTHORIZATION])
        .allow_credentials(true)
        .expose_headers([http::header::SET_COOKIE]);

    let app = Router::new()
        .route("/", get(app_endpoint))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/foo", get(handler_1)
        .route_layer(middleware::from_fn_with_state(state.clone(),auth_middleware))
    )
        .merge(user_routes(state.clone()))
        // .layer(CatchPanicLayer::custom({
        //     let pool = state.pool.clone();
        //     move |error| {
        //         let pool = pool.clone();
        //         tokio::task::spawn_blocking(move || {
        //             // Handle the error in a blocking task
        //             tokio::runtime::Handle::current().block_on(async {
        //                 handle_error(error, &pool).await;
        //             });
        //         });
        //         // Return a placeholder response
        //         Response::new("An error occurred".to_string())
        // }}))
        //.route_layer(middleware::from_fn_with_state(state.clone(), my_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), error_middleware))
        //.layer(middleware::from_fn(add_cors_headers))
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
        //.layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(layer)
        .with_state(state)
        //.fallback(fallback_handler)
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000 ...");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

// async fn add_cors_headers(request: Request, next: Next) -> impl IntoResponse {
//     let mut response = next.run(request).await;
//     response.headers_mut().insert(
//         http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
//         HeaderValue::from_static("true"),
//     );
//     response
// }

// ####

// use hyper::{client::conn::http1, service::service_fn};
// use std::net::SocketAddr;
// use tokio::net::TcpStream;

// async fn proxy_to_dioxus(mut req: Request<Body>) -> impl IntoResponse {
//     // Construct the URI for the Dioxus server
//     let uri_str = format!(
//         "http://127.0.0.1:8080{}",
//         req.uri()
//             .path_and_query()
//             .map(|x| x.as_str())
//             .unwrap_or("/")
//     );
//     *req.uri_mut() = uri_str.parse::<Uri>().unwrap();

//     // Connect to the Dioxus server
//     let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
//     match TcpStream::connect(addr).await {
//         Ok(stream) => {
//             // Perform the HTTP handshake
//             match http1::handshake(stream).await {
//                 Ok((mut sender, connection)) => {
//                     // Spawn a task to poll the connection
//                     tokio::spawn(async move {
//                         if let Err(err) = connection.await {
//                             eprintln!("Connection error: {:?}", err);
//                         }
//                     });

//                     // Send the request and await the response
//                     match sender.send_request(req).await {
//                         Ok(response) => response,
//                         Err(err) => {
//                             eprintln!("Request error: {:?}", err);
//                             axum::http::Response::builder()
//                                 .status(502)
//                                 .body(Body::from("Bad Gateway"))
//                                 .unwrap()
//                         }
//                     }
//                 }
//                 Err(err) => {
//                     eprintln!("Handshake error: {:?}", err);
//                     axum::http::Response::builder()
//                         .status(502)
//                         .body(Body::from("Bad Gateway"))
//                         .unwrap()
//                 }
//             }
//         }
//         Err(err) => {
//             eprintln!("Connection error: {:?}", err);
//             axum::http::Response::builder()
//                 .status(502)
//                 .body(Body::from("Bad Gateway"))
//                 .unwrap()
//         }
//     }
// }

// ###

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
