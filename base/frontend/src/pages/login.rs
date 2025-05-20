use crate::{
    models::{AddUser, UserResponse},
    Route,
};
use dioxus::{logger::tracing::info, prelude::*};
use gloo_net::http::Request;
use sam_ui::popup::*;
use sam_util::validators::{validate_email, validate_password};
use web_sys::RequestCredentials;

#[component]
pub fn LoginPage() -> Element {
    // let mut popup_state = use_signal(|| PopupState::new());
    let mut popup_state = use_signal(|| PopupState::Close);
    let mut msg = use_signal(|| MsgConfig::default());
    let mut success_msg = use_signal(|| MsgConfig::default());
    let mut spinner_state = use_signal(|| PopupState::Close);

    let mut test = use_signal(|| false);

    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut show_errors = use_signal(|| false);
    let mut email_error = use_signal(|| None);
    let mut password_error = use_signal(|| None);
    let mut is_valid = use_signal(|| false);

    let user_state = use_context::<crate::models::SharedUserState>();

    let mut update_errors_state = move || {
        email_error.set(validate_email(email().as_str()).err());
        password_error.set(validate_password(password().as_str(), 8).err());
        is_valid.set(email_error().is_none() && password_error().is_none());
        show_errors.set(true);
    };

    rsx! {
        Link { to: Route::Home {}, "go home" }
        div {
            div {
                input {
                    value: email,
                    class: if show_errors() && email_error().is_some() { "border border-red-500" },
                    class: "input",
                    oninput: move |evt| {
                        show_errors.set(false);
                        email.set(evt.value())
                    },
                    placeholder: "email",
                }

                if show_errors() && email_error().is_some() {
                    p { class: "text-red-500 px-4", "{email_error().unwrap()}" }
                }
            }
            div {
                input {
                    value: password,
                    class: if show_errors() && password_error().is_some() { "border border-red-500" },
                    class: "input",
                    oninput: move |evt| {
                        show_errors.set(false);
                        password.set(evt.value())
                    },
                    placeholder: "password",
                }

                if show_errors() && password_error().is_some() {
                    p { class: "text-red-500 px-4", "{password_error().unwrap()}" }
                }
            }
            button {
                class: "btn",
                onclick: move |_| {
                    spinner_state.set(PopupState::Open);
                    update_errors_state();
                    let user_state = user_state.clone();
                    async move {
                        let user = AddUser {
                            email: email(),
                            password: password(),
                        };
                        let url = format!("{}/users/login", crate::constants::BASE_URL);
                        match Request::post(&url)
                            .header("Content-Type", "application/json")
                            .credentials(RequestCredentials::Include)
                            .json(&user)
                            .unwrap()
                            .send()
                            .await
                        {
                            Ok(res) => {
                                spinner_state.set(PopupState::Close);
                                let json: UserResponse = res.json().await.unwrap();
                                if res.ok() {
                                    user_state.borrow_mut().loggedin = true;
                                    success_msg.set(MsgConfig::with_success(json.message));
                                } else {
                                    msg.set(MsgConfig::with_err(json.message));
                                }
                            }
                            Err(e) => {
                                spinner_state.set(PopupState::Close);
                                msg.set(MsgConfig::with_err(e.to_string()));
                            }
                        }
                    }
                },
                "Login"
            }

            button {
                class: "btn-sec ml-3",
                onclick: move |_| {
                    msg.set(MsgConfig::with_err("errrr!!!!!!"));
                },
                "err"
            }
            button {
                class: "btn-sec ml-3",
                onclick: move |_| {
                    msg.set(
                        MsgConfig::with_success(
                            format!(
                                "Congratulation!!!!!\nsamoora\nahmadddlkjlkjlkj lkjlkjlkj lkjlkjlkj",
                            ),
                        ),
                    );
                },
                "success"
            }
            button {
                class: "btn-sec ml-3",
                onclick: move |_| {
                    msg.set(MsgConfig::with_confirm("You will delete "));
                },
                "confirm"
            }

            Popup {
                state: popup_state,
                update_popup_class: "bg-sky-300",
                enter_anim_class: "animate__animated animate__zoomIn",
                leave_anim_class: "animate__animated animate__zoomOut",
                children: rsx! {
                    button {
                        onclick: move |_| {
                            popup_state.set(PopupState::CloseWithAnimation);
                        },
                        "cancel"
                    }
                },
            }

            {Toast(success_msg())}
            {Msg(msg())}
            Spinner { state: spinner_state }
        }
    }
}

// err_popup_props
//     .set(
//         sam_ui::popup::ErrorMsgProps::new()
//             .title("title")
//             .message(json.message),
//     );

// use dioxus::logger::tracing::info;
// use dioxus::prelude::*;
// use sam_util::validators::{validate_email, validate_password};

// #[component]
// pub fn LoginPage() -> Element {
//     let mut email = use_signal(|| "".to_string());
//     let mut password = use_signal(|| "".to_string());
//     let mut show_errors = use_signal(|| false);
//     let mut has_err_list = use_signal(|| vec![true, true]);
//     let valid = use_memo(move || !has_err_list().contains(&true));

//     rsx! {
//         div {
//             div {
//                 input {
//                     value: email,
//                     class: "input",
//                     oninput: move |evt: FormEvent| email.set(evt.value()),
//                     placeholder: "email",
//                 }

//                 // Error message view
//                 if show_errors() {
//                     match validate_email(email().as_str()) {
//                         Err(msg) => {
//                             has_err_list.with_mut(|ls| ls[0] = true);
//                             rsx! {
//                                 p { class: "text-red-500 px-4", "{msg}" }
//                             }
//                         }
//                         _ => {
//                             has_err_list.with_mut(|ls| ls[0] = false);
//                             rsx! {}
//                         }
//                     }
//                 }
//             }
//             div {
//                 input {
//                     value: password,
//                     class: "input",
//                     oninput: move |evt| password.set(evt.value()),
//                     placeholder: "password",
//                 }

//                 // Error message view
//                 if show_errors() {
//                     match validate_password(password().as_str(), 8) {
//                         Err(msg) => {
//                             has_err_list.with_mut(|ls| ls[1] = true);
//                             rsx! {
//                                 p { class: "text-red-500 px-4", "{msg}" }
//                             }
//                         }
//                         _ => {
//                             has_err_list.with_mut(|ls| ls[1] = false);
//                             rsx! {}
//                         }
//                     }
//                 }
//             }
//             button {
//                 class: "btn",
//                 onclick: move |_| {
//                     show_errors.set(true);
//                     info!("{}", valid());
//                 },
//                 "Login"
//             }
//         }
//     }
// }
