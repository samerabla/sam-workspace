use axum::{
    body::Body,
    http::{HeaderValue, Request},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use dioxus::prelude::*;
use hyper::{
    client::conn::http1,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
    service::service_fn,
    Method,
};
use hyper_util::rt::tokio::TokioIo; // Add this!
use tower_http::cors::{Any, CorsLayer};

use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on http://127.0.0.1:3000");

    axum::serve(
        listener,
        Router::new()
            .route("/", get(ssr_home))
            .route("/contact", get(ssr_home))
            .fallback(axum::routing::any(proxy_to_dioxus_dev))
            .layer(
                CorsLayer::new()
                    .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::GET, Method::POST])
                    .allow_headers([ORIGIN, CONTENT_TYPE, ACCEPT, AUTHORIZATION])
                    .allow_credentials(true),
            ),
    )
    .await
    .unwrap();
}

async fn fallback_handler(req: Request<Body>) -> impl IntoResponse {
    proxy_to_dioxus_dev(req).await
}

async fn ssr_home() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head><title>My App</title></head>
        <body>
            <div id="main"></div>
            <script src="/assets/main.js"></script> <!-- Dioxus will hydrate here -->
        </body>
        </html>
    "#,
    )
}

use http_body_util::BodyExt;
use hyper::{body::Incoming, Response};

async fn proxy_to_dioxus_dev(req: Request<Body>) -> impl IntoResponse {
    let addr = "127.0.0.1:8080";

    match TcpStream::connect(addr).await {
        Ok(stream) => match http1::handshake(TokioIo::new(stream)).await {
            Ok((mut sender, conn)) => {
                tokio::spawn(async move {
                    if let Err(err) = conn.await {
                        eprintln!("connection error: {err:?}");
                    }
                });

                match sender.send_request(req).await {
                    Ok(resp) => map_hyper_to_axum_response(resp).await,
                    Err(_) => {
                        (axum::http::StatusCode::BAD_GATEWAY, "Proxy send failed").into_response()
                    }
                }
            }
            Err(_) => {
                println!("req fat ...");
                (axum::http::StatusCode::BAD_GATEWAY, "Handshake failed").into_response()
            }
        },
        Err(_) => (axum::http::StatusCode::BAD_GATEWAY, "Connect failed").into_response(),
    }
}

async fn map_hyper_to_axum_response(resp: Response<Incoming>) -> axum::response::Response {
    let (parts, body) = resp.into_parts();
    let bytes = body.collect().await.unwrap().to_bytes(); // <-- Collect full body
    let body = Body::from(bytes);

    Response::from_parts(parts, body)
}

fn app() -> Element {
    rsx! {
        // div { "hello world!!!!!" }
        Router::<Route> {}
    }
}
async fn app_endpoint() -> Html<String> {
    // create a component that renders a div with the text "hello world"
    // create a VirtualDom with the app component
    let mut app = VirtualDom::new(app);
    // rebuild the VirtualDom before rendering
    app.rebuild_in_place();

    // render the VirtualDom to HTML
    Html(dioxus_ssr::render(&app))
}

#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq)]
enum Route {
    // Wrap the app in a Nav layout
    #[layout(Nav)]
        #[route("/")]
        Homepage {},

        #[route("/blog/:id")]
        Blog { id: String },
}

#[component]
fn Homepage() -> Element {
    rsx! {
        h1 { "Welcome home" }
    }
}

#[component]
fn Blog(id: String) -> Element {
    rsx! {
        h1 { "How to make: " }
        p { "{id}" }
    }
}

/// A simple nav bar that links to the homepage and blog pages
///
/// The `Route` enum gives up typesafe routes, allowing us to rename routes and serialize them automatically
#[component]
fn Nav() -> Element {
    rsx! {
        nav {
            li {
                Link { to: Route::Homepage {}, "Go home" }
            }
            li {
                Link {
                    to: Route::Blog {
                        id: "Brownies".to_string(),
                    },
                    onclick: move |_| { println!("Clicked on Brownies") },
                    "Learn Brownies"
                }
            }
            li {
                Link {
                    to: Route::Blog {
                        id: "Cookies".to_string(),
                    },
                    "Learn Cookies"
                }
            }
        }
        div { Outlet::<Route> {} }
    }
}
