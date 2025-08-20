use dioxus::{logger::tracing::info, prelude::*};
use input::*;
use route::Route;
use sam_util::fetch_data;
use shared::{
    dashboard::DashNavItemInfo,
    user::{SharedUserState, UserInfo, UserResponse, UserState},
};
use std::{cell::RefCell, rc::Rc};

mod components;
mod enviroment;
mod input;
mod pages;
mod route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND: Asset = asset!("/assets/tailwind.css");

// #[derive(Debug, Clone, Routable, PartialEq)]
// #[rustfmt::skip]
// enum Route {
//     #[route("/")]
//     Home {},
//     #[route("/login")]
//     LoginPage {},
//     #[route("/dashboard")]
//     Dashboard {}
// }

#[component]
pub fn App() -> Element {
    // Check auth and initialize the shared user state
    let user_resource: Resource<Option<UserInfo>> = use_resource(move || async move {
        let url = format!("{}/users/check-auth", crate::enviroment::BASE_URL);
        match fetch_data(&url).await {
            Ok(res) => match res.json::<UserResponse>().await {
                Ok(user_res) => {
                    if user_res.has_json() {
                        Some(serde_json::from_value::<UserInfo>(user_res.json().unwrap()).unwrap())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
            Err(_) => None,
        }
    });

    let mut user_state: Signal<Rc<RefCell<UserState>>> = use_signal(|| {
        let user_data = user_resource.read_unchecked().clone().flatten();
        Rc::new(RefCell::new(UserState::new(user_data.map(|u| u.email))))
    });

    use_effect(move || {
        match &*user_resource.read_unchecked() {
            Some(Some(user)) => {
                user_state.with_mut(|state| {
                    let mut state = state.borrow_mut();
                    state.email = Some(user.email.clone());
                    state.loading = false;
                });
            }
            Some(None) => {
                user_state.with_mut(|state| {
                    state.borrow_mut().loading = false;
                });
            }
            None => {} // Still loading
        }
    });

    use_context_provider::<Signal<Rc<RefCell<UserState>>>>(move || user_state);

    let dash_nav_items = use_signal(|| Rc::new(RefCell::new(Vec::new())));
    use_context_provider::<Signal<Rc<RefCell<Vec<DashNavItemInfo>>>>>(move || dash_nav_items);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: "https://cdnjs.cloudflare.com/ajax/libs/animate.css/4.1.1/animate.min.css" }

        document::Link { rel: "stylesheet", href: TAILWIND }
        Router::<Route> {}
    }
}
