use sam_util::{get_session_storage, remove_session_storage, set_session_storage};
use std::{cell::RefCell, rc::Rc};

use crate::{components::Guard, route::Route};
use dioxus::{logger::tracing::info, prelude::*};
use dioxus_html::nav;
use sam_util::fetch_data;
use shared::user::UserResponse;
use shared::{
    dashboard::{DashNavItem, DashNavItemInfo},
    user::UserRole,
};

#[component]
pub fn DashboardPage() -> Element {
    let nav_items = use_context::<Signal<Rc<RefCell<Vec<DashNavItemInfo>>>>>();
    let path: Route = use_route();

    use_effect(move || {
        if nav_items.read().borrow().is_empty() {
            let nav = use_navigator();
            set_session_storage("redirect", path.to_string().as_str());
            nav.push(Route::DashboardMiddleware {});
        }
    });

    rsx! {
        Guard { redirect_to: "/dashboard".to_string(),
            div { class: "dashboard flex",
                // Pass the nav items as props to avoid re-fetching
                DashboardNavbar { nav_items: nav_items.read().borrow().clone() }
                div { class: "content", Outlet::<Route> {} }
            }
        }
    }
}

#[component]
pub fn DashboardHomepage() -> Element {
    rsx! {
        h1 { "Dashboard Home" }
    }
}

#[component]
pub fn DashboardMiddleware() -> Element {
    let mut nav_items = use_context::<Signal<Rc<RefCell<Vec<DashNavItemInfo>>>>>();

    // Move the resource here so it's only created once for the entire dashboard
    let dash_nav_items: Resource<Vec<DashNavItemInfo>> = use_resource(move || async move {
        let url = format!("{}/users/dashboard/nav-items", crate::enviroment::BASE_URL);
        match fetch_data(&url).await {
            Ok(res) => match res.json::<UserResponse>().await {
                Ok(user_res) => {
                    if user_res.has_json() {
                        let items = serde_json::from_value::<Vec<DashNavItemInfo>>(
                            user_res.json().unwrap(),
                        )
                        .unwrap_or_default();
                        items
                    } else {
                        vec![]
                    }
                }
                Err(_) => vec![],
            },
            Err(_) => vec![],
        }
    });

    use_effect(move || {
        match &*dash_nav_items.read_unchecked() {
            Some(d_n_items) => {
                nav_items.with_mut(|items| {
                    let mut items = items.borrow_mut();
                    items.clear();
                    items.extend(d_n_items.clone());
                    let nav = use_navigator();
                    if !items.is_empty() {
                        if let Some(redirect) = get_session_storage("redirect") {
                            nav.push(redirect);
                            remove_session_storage("redirect");
                        } else {
                            nav.push(items[0].route.clone());
                            // nav.push(Route::DashboardHomepage {});
                        }
                    }
                });
            }
            None => {} // Still loading
        }
    });

    rsx! {
        Guard { redirect_to: "/dashboard-middleware" }
        h1 { "Loading...." }
    }
}

#[component]
pub fn DashboardNavbar(nav_items: Vec<DashNavItemInfo>) -> Element {
    rsx! {
        nav { class: "dashboard-navbar w-64 bg-gray-800 min-h-screen",
            ul {
                for item in nav_items.iter() {
                    li {
                        key: "{item.route}",
                        class: "nav-item bg-primary-500 text-white p-2",
                        Link {
                            to: "{item.route}",
                            class: "nav-link block w-full text-left",
                            "{item.name}"
                        }
                    }
                }
            }
        }
    }
}
