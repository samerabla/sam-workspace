use dioxus::prelude::*;
use shared::user::SharedUserState;

use crate::route::Route;

#[component]
pub fn Guard(children: Element, redirect_to: String) -> Element {
    let user_state = use_context::<Signal<SharedUserState>>();

    // Check if we're still loading the user state
    if user_state().borrow().is_loading() {
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
