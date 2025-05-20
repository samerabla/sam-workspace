use std::{cell::RefCell, rc::Rc};

use dioxus::{logger::tracing::info, prelude::*};
use input::*;
use models::UserState;
use pages::*;

mod constants;
mod input;
pub mod models;
mod pages;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    LoginPage {}
}

#[component]
pub fn App() -> Element {
    let user_state = Rc::new(RefCell::new(UserState::default()));
    use_context_provider(move || user_state.clone());

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: "https://cdnjs.cloudflare.com/ajax/libs/animate.css/4.1.1/animate.min.css" }

        document::Link { rel: "stylesheet", href: TAILWIND }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let x = use_context::<models::SharedUserState>();
    info!("from home: {:#?}", x.borrow());

    rsx! {
        h1 { "Home Page hhhhh" }
        LoginForm {}
        SignUpForm {}
    }
}
