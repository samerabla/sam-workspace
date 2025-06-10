#![allow(non_snake_case)]
use dioxus::{logger::tracing::info, prelude::*};
use gloo_timers::future::TimeoutFuture;
use sam_icon::icon;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::Array;

const DEFAULT_BACKDROP_CLASS: &str =
    "fixed inset-0 bg-white opacity-20 flex items-center justify-center z-50";
const DEFAULT_POPUP_CLASS: &str =
    "absolute top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] bg-white p-6 rounded-lg sam-shadow w-max min-w-[300px] max-w-[90%] z-51";

#[derive(Debug, PartialEq, Clone, Props)]
pub struct PopupProps {
    state: Signal<PopupState>,
    children: Element,

    /// Classes to update the popup style
    update_popup_class: Option<String>,

    /// Classes to replace the popup style
    replace_popup_class: Option<String>,

    /// Classes to update the backdrop style
    update_backdrop_class: Option<String>,

    /// Classes to replace the backdrop style
    replace_backdrop_class: Option<String>,
    enter_anim_class: Option<String>,
    leave_anim_class: Option<String>,
    #[props(default = true)]
    close_on_bg_click: bool,
    #[props(default = true)]
    has_backdrop: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum PopupState {
    Open,
    #[default]
    Close,
    CloseWithAnimation,
}

pub fn Popup(mut props: PopupProps) -> Element {
    let mut target = use_signal(|| None);
    let id = use_memo(move || sam_util::gen_id!());

    let backdrop_class = props.replace_backdrop_class.unwrap_or(format!(
        "{DEFAULT_BACKDROP_CLASS} {}",
        props.update_backdrop_class.unwrap_or_default()
    ));
    // let update_backdrop_class = format!(
    //     "{DEFAULT_BACKDROP_CLASS} {}",
    //     props.update_backdrop_class.unwrap_or_default()
    // );
    let popup_class = props.replace_popup_class.unwrap_or(format!(
        "{DEFAULT_POPUP_CLASS} {}",
        props.update_popup_class.unwrap_or_default()
    ));
    let enter_anim_class = props.enter_anim_class.unwrap_or_default();
    let leave_anim_class = props.leave_anim_class.unwrap_or_default();

    if *props.state.read() == PopupState::Close {
        return rsx!();
    }
    if *props.state.read() == PopupState::CloseWithAnimation {
        crate::animation::animate(
            target(),
            &leave_anim_class,
            None,
            Some(move || {
                props.state.set(PopupState::Close);
            }),
        );
    }

    rsx!(
        div { id: id(),
            if props.has_backdrop {
                div {
                    class: "{backdrop_class}",
                    onclick: move |_| {
                        if props.close_on_bg_click {
                            props.state.set(PopupState::CloseWithAnimation);
                        }
                    },
                }
            }
            crate::AnimateWrapper {
                from: "{popup_class}",
                to: "{enter_anim_class}",
                to_style: "animation-duration:500ms",
                onmounted: move |e: web_sys::Element| target.set(Some(e)),
                children: rsx! {
                    {props.children}
                },
            }

        }
    )
}

// pub struct ErrorPopup;

// impl ErrorPopup {
//     pub fn show() -> Self {
//         Self {}
//     }
// }

#[derive(Debug, PartialEq, Clone, Props, Default)]
pub struct MsgConfig {
    pub state: Signal<PopupState>,
    pub title: String,
    pub message: String,
    pub ty: MsgType,

    /// Callback to be executed on clicking ok button in a confiem msg
    pub callback: Option<Callback<()>>,

    /// Alternative text for ok button
    pub ok_text: Option<String>,

    /// Alternative text for cancel button
    pub cancel_text: Option<String>,
}

impl MsgConfig {
    pub fn with_err(msg: impl AsRef<str>) -> Self {
        Self::default()
            .title("Error!")
            .message(msg.as_ref())
            .ty(MsgType::Err)
            .show()
    }

    pub fn with_success(msg: impl AsRef<str>) -> Self {
        Self::default()
            .title("Success!")
            .message(msg.as_ref())
            .ty(MsgType::Success)
            .show()
    }

    pub fn with_confirm(msg: impl AsRef<str>) -> Self {
        Self::default()
            .title("Are you sure?")
            .message(msg.as_ref())
            .ty(MsgType::Confirm)
            .show()
    }

    pub fn callback(mut self, mut f: impl FnMut() + 'static) -> Self {
        let callback = Callback::new(move |()| f());
        self.callback = Some(callback);
        self
    }

    pub fn title(mut self, title: impl AsRef<str>) -> Self {
        self.title = title.as_ref().to_string();
        self
    }
    pub fn message(mut self, message: impl AsRef<str>) -> Self {
        self.message = message.as_ref().to_string();
        self
    }
    pub fn ok_text(mut self, text: impl Into<String>) -> Self {
        self.ok_text = Some(text.into());
        self
    }
    pub fn cancel_text(mut self, text: impl Into<String>) -> Self {
        self.cancel_text = Some(text.into());
        self
    }

    fn ty(mut self, ty: MsgType) -> Self {
        self.ty = ty;
        self
    }

    fn show(mut self) -> Self {
        self.state.set(PopupState::Open);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum MsgType {
    Success,
    #[default]
    Err,
    Confirm,
}

impl MsgType {
    pub fn class<T>(&self, classes: T) -> &str
    where
        T: AsRef<[&'static str]>,
    {
        let classes = classes.as_ref();
        match self {
            MsgType::Success => classes.get(0).unwrap_or(&""),
            MsgType::Err => classes.get(1).unwrap_or(&""),
            MsgType::Confirm => classes.get(2).unwrap_or(&""),
        }
    }

    pub fn icon(&self) -> Element {
        match self {
            MsgType::Success => icon!(LdCircleCheck, 60, "white", "var(--color-green-500)"),
            MsgType::Err => icon!(LdOctagonX, 60, "white", "var(--color-red-500)"),
            MsgType::Confirm => {
                icon!(
                    LdMessageCircleWarning,
                    60,
                    "white",
                    "var(--color-primary-500)"
                )
            }
        }
    }
}

/// Split the message property by \n to show it in multi lines
/// # example
/// ```
/// let msg_config = use_signal(|| MsgConfig::default())
/// rsx!(
///   button {
///           onclick: move |_| {
///               msg.set(MsgConfig::with_err("Oops!!!!!!"));
///           },
///           "show error message"
///         }
///  button {
///           onclick: move |_| {
///               msg.set(MsgConfig::with_success("Congratulation!!!!!!"));
///           },
///           "show success message"
///          }
///  button {
///           onclick: move |_| {
///               msg.set(MsgConfig::default());
///           },
///           "Close Message"
///          }
///  {Msg(msg_config())}
/// )
/// ```
#[component]
pub fn Msg(mut props: MsgConfig) -> Element {
    let color = props
        .ty
        .class(["text-green-500", "text-red-500", "text-primary-500"]);
    let shadow = props
        .ty
        .class(["sam-shadow-green", "sam-shadow-red", "sam-shadow"]);
    let btn = props.ty.class(["btn-success", "btn-err", "btn"]);
    let ok = props.ok_text.unwrap_or("Ok".to_string());
    let cancel = props.cancel_text.unwrap_or("Cancel".to_string());

    rsx! {
        Popup {
            state: props.state,
            update_popup_class: "{shadow}",
            enter_anim_class: "animate__animated animate__zoomIn",
            leave_anim_class: "animate__animated animate__zoomOut",
            children: rsx! {
                div {
                    tabindex: "0",
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            if let Some(c) = props.callback {
                                props.state.set(PopupState::CloseWithAnimation);
                                c.call(());
                            } else {
                                props.state.set(PopupState::CloseWithAnimation);
                            }
                        } else if e.key() == Key::Escape {
                            props.state.set(PopupState::CloseWithAnimation);
                        }
                    },
                    onmounted: move |e| {
                        use dioxus_web::WebEventExt;
                        if let Some(el) = e.data().as_web_event().dyn_into::<web_sys::HtmlElement>().ok()
                        {
                            el.focus().ok();
                        }
                    },

                    div { class: "center text-primary-500", {props.ty.icon()} }
                    h1 { class: "font-deplomata text-center text-2xl m-2.5 {color}", "{props.title}" }
                    {
                        let lines: Vec<&str> = props.message.split("\n").collect();
                        rsx! {
                            for text in lines {
                                p { class: "text-center", "{text}" }
                            }
                        }
                    }
                    if props.ty == MsgType::Confirm {
                        div { class: "flex justify-around m-8",
                            button {
                                class: "btn-sec mr-2.5",
                                onclick: move |_| {
                                    props.state.set(PopupState::CloseWithAnimation);
                                },
                                "{cancel}"
                            }
                            button {
                                class: "btn",
                                onclick: move |_| {
                                    match props.callback {
                                        Some(c) => c.call(()),
                                        None => info!("You didn't provide a callback"),
                                    };
                                    props.state.set(PopupState::CloseWithAnimation);
                                },
                                "{ok}"
                            }
                        }
                    } else {
                        div { class: "center m-5",
                            button {
                                class: "{btn}",
                                onclick: move |_| {
                                    props.state.set(PopupState::CloseWithAnimation);
                                },
                                "Ok"
                            }
                        }
                    }
                }
            },
        }
    }
}

// #[component]
// pub fn ErrorMsg(mut props: MsgConfig) -> Element {
//     rsx! {
//         Popup {
//             state: props.state,
//             update_popup_class: "sam-shadow-red",
//             enter_anim_class: "animate__animated animate__zoomIn",
//             leave_anim_class: "animate__animated animate__zoomOut",
//             children: rsx! {
//                 div { class: "center", {icon!(LdOctagonX, 60, "white", "red")} }

//                 h1 { class: "font-deplomata text-red-600 text-center text-2xl m-2.5", "{props.title}" }
//                 p { class: "text-center", "{props.message}" }
//                 div { class: "center m-5",
//                     button {
//                         class: "btn-error",
//                         onclick: move |_| {
//                             props.state.set(PopupState::CloseWithAnimation);
//                         },
//                         "Ok"
//                     }

//                 }
//             },
//         }
//     }
// }

// #[component]
// pub fn SuccessMsg(mut props: MsgConfig) -> Element {
//     rsx! {
//         Popup {
//             state: props.state,
//             update_popup_class: "sam-shadow-red",
//             enter_anim_class: "animate__animated animate__zoomIn",
//             leave_anim_class: "animate__animated animate__zoomOut",
//             children: rsx! {
//                 div { class: "center", {icon!(LdCircleCheck, 60, "white", "#00a63e")} }

//                 h1 { class: "font-deplomata text-green-600 text-center text-2xl m-2.5", "{props.title}" }
//                 p { class: "text-center", "{props.message}" }
//                 div { class: "center m-5",
//                     button {
//                         class: "btn-success",
//                         onclick: move |_| {
//                             props.state.set(PopupState::CloseWithAnimation);
//                         },
//                         "Ok"
//                     }

//                 }
//             },
//         }
//     }
// }

#[component]
pub fn Toast(mut props: MsgConfig) -> Element {
    if *props.state.read() == PopupState::Open {
        spawn(async move {
            TimeoutFuture::new(2_000).await;
            props.state.set(PopupState::CloseWithAnimation);
        });
    }
    rsx! {
        Popup {
            state: props.state,
            has_backdrop: false,
            replace_popup_class: "fixed sam-shadow bottom-6 right-6 bg-green-500 text-white rounded-full min-w-[400px] max-w-[90%] h-[50px] z-51",
            enter_anim_class: "animate__animated animate__slideInRight",
            leave_anim_class: "animate__animated animate__slideOutRight",
            children: rsx! {
                div { class: "flex h-[50px]",
                    div { class: "center pl-1.5", {icon!(LdCircleCheck, 40, "var(--color-green-500)", "white")} }
                    div { class: "center grow justify-start pl-1.5",

                        p { class: "text-center", "{props.message}" }
                    }
                    div { class: "center m-6",
                        button {
                            class: "text-white text-l cursor-pointer",
                            onclick: move |_| {
                                props.state.set(PopupState::CloseWithAnimation);
                            },
                            "X"
                        }
                    }

                }
            },
        }
    }
}

// TODO >>
// Handle AninmationProps and covert all str into Option<str> in order to be able to not provide from or to classes

// #[derive(Clone, Debug, PartialEq, Props)]
// pub struct PopupSpinnerConfig {
//     state: Signal<PopupState>,
//     config: crate::spinner::SpinnerConfig,
// }

// #[component]
// pub fn Spinner(props: PopupSpinnerConfig) -> Element {
//     rsx! {
//         Popup {
//             state: props.state,
//             close_on_bg_click: false,
//             replace_popup_class: "absolute top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] z-51",
//             update_backdrop_class: "opacity-80",
//             enter_anim_class: "TODO",
//             leave_anim_class: "TODO",
//             children: rsx! {
//                 div { class: "center", {crate::spinner::Spinner(props.config)} }
//             },
//         }
//     }
// }

#[component]
pub fn Spinner(state: Signal<PopupState>) -> Element {
    rsx! {
        Popup {
            state,
            close_on_bg_click: false,
            replace_popup_class: "absolute top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] z-51",
            update_backdrop_class: "opacity-80",
            enter_anim_class: "TODO",
            leave_anim_class: "TODO",
            children: rsx! {
                img { src: asset!("/assets/spinner.svg") }
            },
        }
    }
}
