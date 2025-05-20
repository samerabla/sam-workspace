use dioxus::{logger::tracing::info, prelude::*};

//**** TODO: use one_cell later when it becomes in std and not in nightly
//use once_cell::sync::Lazy;
// static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
//     Regex::new(r"^[\w.+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap()
// });

#[derive(Debug)]
struct LoginData {
    email: String,
    password: String,
}

#[component]
pub fn LoginForm() -> Element {
    let mut email: Signal<String> = use_signal(|| "".to_string());
    let password: Signal<String> = use_signal(|| "".to_string());
    let mut email_err_msg: Signal<InputErrorMsg> = use_signal(|| InputErrorMsg::default());
    let mut password_err_msg: Signal<InputErrorMsg> = use_signal(|| InputErrorMsg::default());
    rsx! {
        div { class: "flex flex-col space-y-4",
            div {
                SamInput {
                    val: email,
                    input_type: Some("email".to_string()),
                    err_msg: email_err_msg,
                    placeholder: "Email".to_string(),
                }
                InputErrorMessageView { err_msg: email_err_msg }
            }
            div {
                SamPasswordInput { val: password, err_msg: password_err_msg }
                InputErrorMessageView { err_msg: password_err_msg }
            }

            button {
                onclick: move |_| {
                    email_err_msg
                        .with_mut(|err| {
                            err.show = true;
                            err.value = validate_email(email.read().as_str());
                        });
                    password_err_msg
                        .with_mut(|err| {
                            err.show = true;
                            err.value = validate_empty(password.read().as_str());
                        });
                    if email_err_msg.read().value.is_none()
                        && password_err_msg.read().value.is_none()
                    {
                        let login_data = LoginData {
                            email: email.read().to_string(),
                            password: password.read().to_string(),
                        };
                        info!("{:#?}", login_data)
                    }
                },
                "submit"
            }
            button { onclick: move |_| email.set("tttt".to_string()), "setoo" }
        }
    }
}

#[component]
pub fn SignUpForm() -> Element {
    let email: Signal<String> = use_signal(|| "".to_string());
    let password: Signal<String> = use_signal(|| "".to_string());
    let comfirm_password: Signal<String> = use_signal(|| "".to_string());
    let mut email_err_msg: Signal<InputErrorMsg> = use_signal(|| InputErrorMsg::default());
    let mut password_err_msg: Signal<InputErrorMsg> = use_signal(|| InputErrorMsg::default());
    let mut comfirm_password_err_msg: Signal<InputErrorMsg> =
        use_signal(|| InputErrorMsg::default());
    rsx! {
        div { class: "flex flex-col space-y-4",
            SamInput {
                val: email,
                input_type: "email".to_string(),
                err_msg: email_err_msg,
                placeholder: "Email".to_string(),
            }
            SamInput {
                val: password,
                input_type: "password".to_string(),
                err_msg: password_err_msg,
                placeholder: "Password".to_string(),
            }

            SamInput {
                val: comfirm_password,
                input_type: "password".to_string(),
                err_msg: comfirm_password_err_msg,
                placeholder: "Comfirm Password".to_string(),
            }

            button {
                onclick: move |_| {
                    email_err_msg
                        .with_mut(|err| {
                            err.show = true;
                            err.value = validate_email(email.read().as_str());
                        });
                    password_err_msg
                        .with_mut(|err| {
                            err.show = true;
                            err.value = validate_password(password.read().as_str());
                        });
                    comfirm_password_err_msg
                        .with_mut(|err| {
                            err.show = true;
                            err.value = validate_comfirm_password(
                                password.read().as_str(),
                                comfirm_password.read().as_str(),
                            );
                        });
                    if email_err_msg.read().value.is_none()
                        && password_err_msg.read().value.is_none()
                        && comfirm_password_err_msg.read().value.is_none()
                    {
                        let login_data = LoginData {
                            email: email.read().to_string(),
                            password: password.read().to_string(),
                        };
                        info!("{:#?}", login_data)
                    }
                },
                "submit"
            }
        }
    }
}

#[component]
fn SamInput(
    mut val: Signal<String>,
    input_type: Option<String>,
    mut err_msg: Signal<InputErrorMsg>,
    placeholder: Option<String>,
) -> Element {
    let on_input = move |e: FormEvent| {
        let value = e.data.value();
        val.set(value.clone());
        err_msg.with_mut(|err| err.show = false);
    };
    // let base_input_style = "w-full h-full rounded-full px-4 py-2 border border-gray-300 bg-gray-50 text-gray-800 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500";

    // let input_style = if err_msg.read().value.is_some() && err_msg.read().show {
    //     format!("{base_input_style} border-red-500")
    // } else {
    //     base_input_style.to_string()
    // };

    rsx! {
        div {
            div { class: "h-10",
                input {
                    value: val,
                    class: "input",
                    oninput: on_input,
                    r#type: input_type.unwrap_or("text".to_string()),
                    placeholder: placeholder.unwrap_or("".to_string()),
                }
            }
        }
    }
}

#[component]
fn SamPasswordInput(
    mut val: Signal<String>,
    mut err_msg: Signal<InputErrorMsg>,
    placeholder: Option<String>,
) -> Element {
    let mut input_type: Signal<String> = use_signal(|| "password".to_string());
    let mut show = use_signal(|| false);

    // let icon = if show() {
    //     crate::sam_icon!(name: FaEye,size:20)
    // } else {
    //     crate::sam_icon!(name: FaEyeSlash,size:20)
    // };

    use_effect(move || {
        if show() {
            input_type.set("text".to_string());
        } else {
            input_type.set("password".to_string());
        };
    });

    rsx! {
        div {
            div { class: "relative h-10",
                SamInput {
                    val,
                    input_type: Some(input_type()),
                    err_msg,
                    placeholder: placeholder.unwrap_or("Password".to_string()),
                }
                div {
                    class: "flex items-center absolute top-0 right-4 h-full cursor-pointer",
                    onclick: move |_| show.toggle(),
                    {"icon"}
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct InputErrorMsg {
    pub value: Option<String>,
    pub show: bool,
}

#[component]
fn InputErrorMessageView(err_msg: Signal<InputErrorMsg>) -> Element {
    rsx! {
        div { class: "px-4",
            if err_msg.read().show {
                if let Some(err) = &err_msg.read().value {
                    p { class: "text-red-500", "{err}" }
                }
            }

        }
    }
}

fn validate_password(password: &str) -> Option<String> {
    let str = "Your password should ".to_string();
    if password.is_empty() {
        return Some("Empty not allowed".to_string());
    } else if password.len() < 8 {
        return Some(str + "be at least 8 characters long");
    } else if !password.chars().any(|c| c.is_uppercase()) {
        return Some(str + "contain at least one uppercase letter");
    } else if !password.chars().any(|c| r"!@#$%^&*(),.?:{}|<>".contains(c)) {
        return Some(str + "contain at least one symbol");
    } else if !password.chars().any(|c| c.is_digit(10)) {
        return Some(str + "contain at least one number");
    } else {
        return None;
    }
}

fn validate_empty(password: &str) -> Option<String> {
    if password.is_empty() {
        return Some("Empty not allowed".to_string());
    } else {
        return None;
    }
}

fn validate_email(email: &str) -> Option<String> {
    // // remove this later when using one_cell
    // let EMAIL_REGEX = Regex::new(r"^[\w.+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap();
    // if email.is_empty() {
    //     return Some("Empty not allowed".to_string());
    // } else if !EMAIL_REGEX.is_match(email) {
    //     return Some("enter a valid email".to_string());
    // } else {
    //     return None;
    // }
    return None;
}

fn validate_comfirm_password(password: &str, confirm_password: &str) -> Option<String> {
    if confirm_password.is_empty() {
        return Some("Empty not allowed".to_string());
    } else if password != confirm_password {
        return Some("not matched".to_string());
    } else {
        return None;
    }
}
