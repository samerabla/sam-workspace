use crate::{
    components::{DashboardNavbar, Guard},
    route::Route,
};
use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn DashboardPage() -> Element {
    rsx! {
        Guard { redirect_to: "/dashboard".to_string(),
            div { class: "dashboard flex",
                DashboardNavbar {}
                div { class: "content", Outlet::<Route> {} }
                "Daahooooo"
            }
        }
    }
}

// TODO: create a page for statistics
#[component]
pub fn DashboardHomepage() -> Element {
    rsx! {}
}
