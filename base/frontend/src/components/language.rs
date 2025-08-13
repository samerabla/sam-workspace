use dioxus::{logger::tracing::info, prelude::*};
use sam_ui::{
    input::{Input, InputAppearance},
    popup::{Msg, MsgConfig, PopupState, Spinner, Toast},
};
use sam_util::post_json;
use shared::user::UserResponse;

#[component]
pub fn AddLanguage() -> Element {
    let mut msg = use_signal(|| MsgConfig::default());
    let mut err_msg = use_signal(|| MsgConfig::default());
    let mut spinner_state = use_signal(|| PopupState::Close);
    rsx! {
        div { class: "categories",
            form {
                class: "flex flex-col gap-4",
                onsubmit: move |evt: FormEvent| {
                    info!("xxxxx! {:?}", evt);
                    spinner_state.set(PopupState::Open);
                    async move {
                        let payload = serde_json::json!(
                            { "id" : "New_Category5", "parent_id" : "land" }
                        );
                        let url = format!("{}/categories", crate::enviroment::BASE_URL);
                        match post_json(&url, &payload).await {
                            Ok(res) => {
                                spinner_state.set(PopupState::Close);
                                let json: UserResponse = res.json().await.unwrap();
                                if res.ok() {
                                    msg.set(MsgConfig::with_success(json.message()));
                                } else {
                                    err_msg.set(MsgConfig::with_err(json.message()));
                                }
                            }
                            Err(e) => {
                                spinner_state.set(PopupState::Close);
                                err_msg.set(MsgConfig::with_err(e.to_string()));
                            }
                        }
                    }
                },
                div { class: "p-5 w-[400px]",
                    Input {
                        name: "id",
                        appearance: InputAppearance::square,
                        label: "Category Code",
                        oninput: move |evt: FormEvent| {},
                    }
                }
                div { class: "p-5 w-[400px]",
                    Input {
                        name: "parent_id",
                        appearance: InputAppearance::rounded,
                        label: "Parent",
                        oninput: move |evt: FormEvent| {},
                    }
                }
                button { r#type: "submit", "Submit" }
            }
            button {
                onclick: move |_| {
                    spinner_state.set(PopupState::Open);
                    async move {
                        let payload = serde_json::json!(
                            { "id" : "New_Category3", "parent_id" : "land" }
                        );
                        let url = format!("{}/categories", crate::enviroment::BASE_URL);
                        match post_json(&url, &payload).await {
                            Ok(res) => {
                                spinner_state.set(PopupState::Close);
                                let json: UserResponse = res.json().await.unwrap();
                                if res.ok() {
                                    msg.set(MsgConfig::with_success(json.message()));
                                } else {
                                    err_msg.set(MsgConfig::with_err(json.message()));
                                }
                            }
                            Err(e) => {
                                spinner_state.set(PopupState::Close);
                                err_msg.set(MsgConfig::with_err(e.to_string()));
                            }
                        }
                    }
                },
                "Add Category"
            }
        }
        {Toast(msg())}
        {Msg(err_msg())}
        Spinner { state: spinner_state }
    }
}
