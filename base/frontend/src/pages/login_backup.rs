use dioxus::{logger::tracing::info, prelude::*};
use gloo_net::http::Request;
use sam_ui::{input::*, popup::*, spinner::SpinnerConfig};
use sam_util::validators::{validate_email, validate_password};
use serde::{Deserialize, Serialize};
use shared::user::*;
use web_sys::RequestCredentials;

#[component]
pub fn LoginPage2() -> Element {
    // let mut popup_state = use_signal(|| PopupState::new());
    let mut popup_state = use_signal(|| PopupState::Close);
    let mut msg = use_signal(|| MsgConfig::default());
    let mut success_msg = use_signal(|| MsgConfig::default());
    let mut spinner_state = use_signal(|| PopupState::Close);

    let mut test = use_signal(|| false);
    let mut tesoooo = use_signal(|| "samoora");

    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut show_errors = use_signal(|| false);
    let mut email_error = use_signal(|| None);
    let mut password_error = use_signal(|| None);
    let mut is_valid = use_signal(|| false);

    let user_state = use_context::<SharedUserState>();

    let mut update_errors_state = move || {
        email_error.set(validate_email(email().as_str()).err());
        password_error.set(validate_password(password().as_str(), 8).err());
        is_valid.set(email_error().is_none() && password_error().is_none());
        show_errors.set(true);
    };

    // Link { to: Route::Home {}, "go home" }
    // h1 { "{tesoooo()}" }
    // class: if show_errors() && email_error().is_some() { "border border-red-500" },
    // class: "input",
    // onmounted: move |e: Event<MountedData>| {
    //     use dioxus_web::WebEventExt;
    //     fm.set(Some(e));
    //     let fm = FormData::new(e)
    // },
    // let pass = match evt.values().get("pass").cloned() {
    //     Some(f) => f.as_value(),
    //     _ => String::new(),
    // };
    let mut fm = use_signal(|| None::<Element>);
    // form {
    //     class: "flex flex-col gap-4",
    //     onsubmit: move |evt: FormEvent| {
    //         info!("pass! {:#?}", (* (evt.data())).parsed_values::< FmVal > ());
    //     },
    //     input { name: "pass" }
    //     button { r#type: "submit", "Submit" }
    // }
    // class: {
    //     let err = if show_errors() && email_error().is_some() {
    //         "border border-red-500"
    //     } else {
    //         ""
    //     };
    //     format!("input {}", err)
    // },
    let mut pass_input_type = use_signal(|| "password");
    rsx! {
        form {
            class: "flex flex-col gap-4",
            onsubmit: move |evt: FormEvent| {
                let data: LoginUserWrapper = evt.into();
                info!("xxxxx! {:#?}", data.0);
            },
            div { class: "p-5 w-[400px]",
                Input {
                    name: "email",
                    appearance: InputAppearance::line,
                    oninput: move |evt: FormEvent| {
                        show_errors.set(false);
                    },
                    label: "Email",
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

                if show_errors() && email_error().is_some() {
                    p { class: "text-red-500 px-4", "{email_error().unwrap()}" }
                }
            }
            button { r#type: "submit", "Submit" }
        }

        hr {}
        sam_ui::spinner::Spinner { size: 20 }
        div { class: "",
            div { class: "p-5 w-[400px]",
                Input {
                    value: email,
                    oninput: move |evt: FormEvent| {
                        show_errors.set(false);
                        email.set(evt.value())
                    },
                    placeholder: "email",
                    label: "Email",
                    animated_label: false,
                }

                if show_errors() && email_error().is_some() {
                    p { class: "text-red-500 px-4", "{email_error().unwrap()}" }
                }
            }
            div { class: "p-5 w-[400px]",
                Input {
                    r#type: pass_input_type(),
                    appearance: InputAppearance::square,
                    oninput: move |evt: FormEvent| {},
                    placeholder: "Type your password",
                    label: "Password",
                    height: 60,
                    animated_label: false,

                    prefix: rsx! {
                        {
                            let mut action = move |_| {
                                pass_input_type
                                    .set(if pass_input_type() == "text" { "password" } else { "text" });
                            };
                            let icon = if pass_input_type() == "text" {
                                sam_icon::icon!(LdEye, 20, "blue", "withe", "red", "white", onclick : action)
                            } else {
                                sam_icon::icon!(
                                    LdEyeOff, 20, "blue", "withe", "red", "white", onclick : action
                                )
                            };
                            icon
                        }
                    },
                    suffix: rsx! {
                        {
                            let mut action = move |_| {
                                pass_input_type
                                    .set(if pass_input_type() == "text" { "password" } else { "text" });
                            };
                            let icon = if pass_input_type() == "text" {
                                sam_icon::icon!(LdEye, 20, "blue", "withe", "red", "white", onclick : action)
                            } else {
                                sam_icon::icon!(
                                    LdEyeOff, 20, "blue", "withe", "red", "white", onclick : action
                                )
                            };
                            icon
                        }
                    },
                }
            }
            div { class: "p-5 w-[400px]",
                Input {
                    appearance: InputAppearance::square,
                    oninput: move |evt: FormEvent| {},
                    placeholder: "email email email v email v v email iiiiiii vvvvvvv",
                    label: "Email 3",
                    height: 90,
                    prefix: rsx! {
                        {
                            let mut action = move |_| {
                                pass_input_type
                                    .set(if pass_input_type() == "text" { "password" } else { "text" });
                            };
                            let icon = if pass_input_type() == "text" {
                                sam_icon::icon!(LdEye, 20, "blue", "withe", "red", "white", onclick : action)
                            } else {
                                sam_icon::icon!(
                                    LdEyeOff, 20, "blue", "withe", "red", "white", onclick : action
                                )
                            };
                            icon
                        }
                    },
                    suffix: rsx! {
                        {
                            let mut action = move |_| {
                                pass_input_type
                                    .set(if pass_input_type() == "text" { "password" } else { "text" });
                            };
                            let icon = if pass_input_type() == "text" {
                                sam_icon::icon!(LdEye, 20, "blue", "withe", "red", "white", onclick : action)
                            } else {
                                sam_icon::icon!(
                                    LdEyeOff, 20, "blue", "withe", "red", "white", onclick : action
                                )
                            };
                            icon
                        }
                    },
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
            div {

                button {
                    class: "btn",
                    onclick: move |_| {
                        spinner_state.set(PopupState::Open);
                        update_errors_state();
                        let user_state = user_state.clone();
                        async move {
                            let user = LoginUser {
                                email: email(),
                                password: password(),
                            };
                            let url = format!("{}/users/login", crate::enviroment::BASE_URL);
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
                                        success_msg.set(MsgConfig::with_success(json.message()));
                                    } else {
                                        msg.set(MsgConfig::with_err(json.message()));
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
                    msg.set(
                        MsgConfig::with_confirm("You will delete ")
                            .cancel_text("tesooooo to tahsinoof xxxx fgdfdgfd cccccccc")
                            .callback(move || tesoooo.set("tahsinooof")),
                    );
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

// impl Into<LoginUser> for FormData {
//     fn into(self) -> LoginUser {
//         let email = self
//             .values()
//             .get("email")
//             .cloned()
//             .unwrap_or_default()
//             .as_value();
//         let password = self
//             .values()
//             .get("password")
//             .cloned()
//             .unwrap_or_default()
//             .as_value();
//         LoginUser { email, password }
//     }
// }

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
