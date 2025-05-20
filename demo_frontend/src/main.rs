use dioxus::{logger::tracing::info, prelude::*};
use gloo_net::http::Request;
use web_sys::RequestCredentials;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    let mut x = use_signal(|| "sam".to_string());
    let f = move |_| async move {
        // let res = reqwest::get("http://localhost:3000/foo")
        //     .await
        //     .unwrap()
        //     .text()
        //     .await
        //     .unwrap();
        match Request::get("http://127.0.0.1:3000/foo")
            .credentials(RequestCredentials::Include)
            .send()
            .await
        {
            Ok(resp) => {
                info!("resp: {:?}", resp.text().await.unwrap());
            }
            Err(e) => info!("{:?}", e),
        }

        // x.set(res);
    };

    let ff = move |_| async move {
        let body = serde_json::json!({
            "email": "samerfamilie@gmail.com",
            "password": "555"
        });
        match Request::post("http://127.0.0.1:3000/vvv")
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .json(&body)
            .unwrap()
            .send()
            .await
        {
            Ok(res) => {
                x.set(format!("Login: {}", res.status()));
            }
            Err(e) => {
                x.set(format!("Login error: {}", e));
            }
        }
    };
    rsx! {
        div { {x} }
        button { onclick: f, "fetch" }
        button { onclick: ff, "Login" }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            // Navigation links
            Link { to: Route::Blog { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::Blog { id: id + 1 }, "Next" }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}
