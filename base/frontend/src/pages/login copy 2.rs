use dioxus::{logger::tracing::info, prelude::*};
use gloo_net::http::Request;
use sam_ui::{input::*, popup::*, spinner::SpinnerConfig};
use sam_util::{
    post_json,
    validators::{validate_email, validate_password},
};
use serde::{Deserialize, Serialize};
use shared::user::*;
use web_sys::RequestCredentials;

use crate::route::Route;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "location"])]
    fn assign(url: &str);
}

#[component]
pub fn LoginPage() -> Element {
    let mut msg = use_signal(|| MsgConfig::default());
    let mut success_msg = use_signal(|| MsgConfig::default());
    let mut spinner_state = use_signal(|| PopupState::Close);
    let mut show_errors = use_signal(|| false);
    let mut email_error = use_signal(|| None);
    let mut password_error = use_signal(|| None);
    let mut is_valid = use_signal(|| false);
    let user_state = use_context::<SharedUserState>();

    //let nav = use_navigator();

    let onsubmit = move |evt: FormEvent| {
        spinner_state.set(PopupState::Open); // Fixed: should be Open, not Close
        let user_state = user_state.clone();
        //let nav = nav.clone();

        spawn(async move {
            let data: LoginUserWrapper = evt.into();
            let user = data.0;

            info!("Login attempt for email: {}", user.email);

            // Validate the form
            email_error.set(validate_email(user.email.as_str()).err());
            password_error.set(validate_password(user.password.as_str(), 8).err());
            is_valid.set(email_error().is_none() && password_error().is_none());
            show_errors.set(true);

            if !is_valid() {
                spinner_state.set(PopupState::Close);
                info!("Validation failed");
                return;
            }

            let url = format!("{}/users/login", crate::enviroment::BASE_URL);
            info!("Sending login request to: {}", url);

            match post_json(&url, &user).await {
                Ok(res) => {
                    info!("Login response received, status: {}", res.status());
                    let json: UserResponse = res.json().await.unwrap();

                    if res.ok() {
                        info!("Login successful");
                        let redirect_to = user_state.borrow().redirect_to.clone();
                        info!("Redirect path: {:?}", redirect_to);

                        // Update user state
                        {
                            let mut user_state_mut = user_state.borrow_mut();
                            user_state_mut.loggedin = true;
                            user_state_mut.email = user.email.clone();
                            user_state_mut.redirect_to = None;
                            info!(
                                "User state updated - logged in: {}",
                                user_state_mut.loggedin
                            );
                        }

                        success_msg.set(MsgConfig::with_success(json.message()));
                        spinner_state.set(PopupState::Close);

                        // Add a small delay to ensure state is properly updated
                        gloo_timers::future::TimeoutFuture::new(4000).await;

                        info!("About to navigate to dashboard");
                        //nav.push(Route::DashboardPage {});
                        // assign("/dashboard");
                        info!("Navigation called");
                    } else {
                        info!("Login failed: {}", json.message());
                        spinner_state.set(PopupState::Close);
                        msg.set(MsgConfig::with_err(json.message()));
                    }
                }
                Err(e) => {
                    info!("Login request failed: {}", e);
                    spinner_state.set(PopupState::Close);
                    msg.set(MsgConfig::with_err(e.to_string()));
                }
            }
        });
    };

    let mut pass_input_type = use_signal(|| "password");

    rsx! {
        form { class: "flex flex-col gap-4", onsubmit,
            div { class: "p-5 w-[400px]",
                Input {
                    name: "email",
                    appearance: InputAppearance::line,
                    oninput: move |evt: FormEvent| {
                        show_errors.set(false);
                    },
                    label: "Email",
                    animated_label: true,
                    prefix: rsx! {
                        {sam_icon::icon!(LdMail, 20, "black", "withe")}
                    },
                }

                if show_errors() && email_error().is_some() {
                    p { class: "text-red-500 px-4", "{email_error().unwrap()}" }
                }
            }

            div { class: "p-5 w-[400px]",
                Input {
                    name: "password",
                    appearance: InputAppearance::line,
                    r#type: pass_input_type(),
                    oninput: move |evt: FormEvent| {
                        show_errors.set(false);
                    },
                    label: "Password",
                    animated_label: true,
                    prefix: rsx! {
                        {sam_icon::icon!(LdLockKeyhole, 20, "var(--color-gray-800)", "withe")}
                    },
                    suffix: rsx! {
                        {
                            let mut action = move |_| {
                                pass_input_type
                                    .set(if pass_input_type() == "text" { "password" } else { "text" });
                            };
                            let icon = if pass_input_type() == "text" {
                                sam_icon::icon!(
                                    LdEye, 20, "var(--color-gray-800)", "withe", "black", "white", onclick :
                                    action
                                )
                            } else {
                                sam_icon::icon!(
                                    LdEyeOff, 20, "var(--color-gray-800)", "withe", "black", "white", onclick
                                    : action
                                )
                            };
                            icon
                        }
                    },
                }

                if show_errors() && password_error().is_some() {
                    p { class: "text-red-500 px-4", "{password_error().unwrap()}" }
                }
            }
            button { class: "btn", r#type: "submit", "Submit" }
        }
        {Toast(success_msg())}
        {Msg(msg())}
        Spinner { state: spinner_state }
    }
}

pub fn Comment(text: &str) -> Element {
    rsx!()
}

#[derive(Debug, Clone, PartialEq, Props)]
struct propso {
    #[props(extends = GlobalAttributes, extends = input)]
    xxx: Vec<Attribute>,
}

#[component]
fn SamInput(props: propso) -> Element {
    rsx!(input { ..props.xxx })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoginForm {
    pub email: Vec<String>,
    pub password: Vec<String>,
}

struct LoginUserWrapper(LoginUser);

impl From<FormEvent> for LoginUserWrapper {
    fn from(evt: FormEvent) -> Self {
        let email = evt
            .data()
            .values()
            .get("email")
            .cloned()
            .unwrap_or_default()
            .as_value();
        let password = evt
            .data()
            .values()
            .get("password")
            .cloned()
            .unwrap_or_default()
            .as_value();
        LoginUserWrapper(LoginUser { email, password })
    }
}
