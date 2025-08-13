use dioxus::prelude::*;
use dioxus_html::g::to;
use sam_ui::popup::{Msg, MsgConfig, PopupState, Spinner};
use sam_util::fetch_data;
use shared::{dashboard::DashNavItemInfo, user::UserResponse};

#[component]
pub fn DashboardNavbar() -> Element {
    let mut msg = use_signal(|| MsgConfig::default());
    let mut err_msg = use_signal(|| MsgConfig::default());
    let mut spinner_state = use_signal(|| PopupState::Close);

    let dash_nav_items: Resource<Vec<DashNavItemInfo>> = use_resource(move || async move {
        let url = format!("{}/users/dashboard/nav-items", crate::enviroment::BASE_URL);
        match fetch_data(&url).await {
            Ok(res) => match res.json::<UserResponse>().await {
                Ok(user_res) => {
                    if user_res.has_json() {
                        serde_json::from_value::<Vec<DashNavItemInfo>>(user_res.json().unwrap())
                            .unwrap()
                    } else {
                        err_msg.set(MsgConfig::with_err(user_res.message()));
                        vec![]
                    }
                }
                Err(e) => {
                    err_msg.set(MsgConfig::with_err(e.to_string()));
                    vec![]
                }
            },
            Err(e) => {
                err_msg.set(MsgConfig::with_err(e.to_string()));
                vec![]
            }
        }
    });

    let items = dash_nav_items
        .read_unchecked()
        .clone()
        .unwrap_or(Vec::new());
    rsx! {
        div { class: "dashboard",
            for item in items.iter() {
                NavbarItem { item: item.clone() }
            }
            {}
        }
        {Msg(err_msg())}
        Spinner { state: spinner_state }
    }
}

#[component]
pub fn NavbarItem(item: DashNavItemInfo) -> Element {
    rsx! {
        div { class: "dashboard",
            Link { to: "{item.route}", class: "nav-link", "{item.name}" }
        }
    }
}
