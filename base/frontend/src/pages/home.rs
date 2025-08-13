use std::{cell::RefCell, rc::Rc};

use dioxus::{logger::tracing::info, prelude::*};
use shared::user::{SharedUserState, UserState};

use crate::input::LoginForm;

#[component]
pub fn HomePage() -> Element {
    let x = use_context::<Signal<SharedUserState>>();
    info!("from home: {:#?}", x().borrow());

    rsx! {
        h1 { "Home Page hhhhh" }
        LoginForm {}
    }
}
