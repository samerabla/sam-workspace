use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{Categories, DashboardNavbar},
    route::Route,
};
use dioxus::{logger::tracing::info, prelude::*};
use shared::user::{SharedUserState, UserState};

// #[component]
// pub fn DashboardPage() -> Element {
//     rsx! {
//         Guard { redirect_to: "/dashboard".to_string(),
//             div { class: "dashboard",
//                 DashboardNavbar {}
//                 div { class: "content", Outlet::<Route> {} }
//             }
//         }
//     }
// }

#[component]
pub fn DashboardPage() -> Element {
    // let user_state = use_context::<SharedUserState>();
    // if !user_state.borrow().loggedin {
    //     user_state.borrow_mut().redirect_to = Some("/dashboard".to_string());
    //     let nav = use_navigator();
    //     nav.push(Route::LoginPage {});
    //     //nav.push(format!("/login?redirect_to=/dashboard"));
    //     return rsx! {
    //         div { "Redirecting to login..." }
    //     };
    // }

    rsx! {
        Guard { redirect_to: "/dashboard".to_string(),
            div { class: "dashboard",
                DashboardNavbar {}
                div { class: "content", Outlet::<Route> {} }
                "Daahooooo"
            }
        }
    }
}

#[component]
pub fn Guard(children: Element, redirect_to: String) -> Element {
    let user_state = use_context::<Signal<Rc<RefCell<UserState>>>>();
    // let user_state = use_context::<SharedUserState>();

    // Check if we're still loading the user state
    if user_state().borrow().is_loading() {
        info!("loddddd");
        return rsx! {
            div { "Loading..." }
        };
    }

    // If not logged in, redirect to login page
    if !user_state().borrow().is_logged_in() {
        user_state.read().borrow_mut().redirect_to = Some(redirect_to);
        let nav = use_navigator();
        nav.push(Route::LoginPage {});
        return rsx! {
            div { "Redirecting to login..." }
        };
    }

    rsx! {
        {children}
    }
}
