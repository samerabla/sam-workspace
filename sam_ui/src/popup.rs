#![allow(non_snake_case)]
use dioxus::{logger::tracing::info, prelude::*};
use gloo_timers::future::TimeoutFuture;
use sam_icon::icon;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Array;

const DEFAULT_OUTER_CLASS: &str =
    "fixed inset-0 bg-white opacity-20 flex items-center justify-center z-50";
const DEFAULT_INNER_CLASS: &str =
    "absolute top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] p-6 rounded-lg sam-shadow min-w-[300px] max-w-[90%] z-51";

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
        "{DEFAULT_OUTER_CLASS} {}",
        props.update_backdrop_class.unwrap_or_default()
    ));
    // let update_backdrop_class = format!(
    //     "{DEFAULT_OUTER_CLASS} {}",
    //     props.update_backdrop_class.unwrap_or_default()
    // );
    let popup_class = props.replace_popup_class.unwrap_or(format!(
        "{DEFAULT_INNER_CLASS} {}",
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

    pub fn title(mut self, title: impl AsRef<str>) -> Self {
        self.title = title.as_ref().to_string();
        self
    }
    pub fn message(mut self, message: impl AsRef<str>) -> Self {
        self.message = message.as_ref().to_string();
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
    let msg_color = match props.ty {
        MsgType::Err => "text-red-600",
        MsgType::Success => "text-green-600",
        MsgType::Confirm => "text-primary",
    };
    rsx! {
        Popup {
            state: props.state,
            update_popup_class: {
                match props.ty {
                    MsgType::Err => "sam-shadow-red",
                    MsgType::Success => "sam-shadow-green",
                    MsgType::Confirm => "sam-shadow",
                }
            },
            enter_anim_class: "animate__animated animate__zoomIn",
            leave_anim_class: "animate__animated animate__zoomOut",
            children: rsx! {
                div { class: "center text-primary",
                    {
                        match props.ty {
                            MsgType::Err => icon!(LdOctagonX, 60, "white", "red"),
                            MsgType::Success => icon!(LdCircleCheck, 60, "white", "#00a63e"),
                            MsgType::Confirm => {
                                icon!(LdMessageCircleWarning, 60, "white", "currentcolor")
                            }
                        }
                    }
                }

                h1 { class: "font-deplomata text-center text-2xl m-2.5 {msg_color}", "{props.title}" }
                {
                    let lines: Vec<&str> = props.message.split("\n").collect();
                    rsx! {
                        for text in lines {
                            p { class: "text-center", "{text}" }
                        }
                    }
                }
                div { class: "center m-5",
                    button {
                        class: match props.ty {
                            MsgType::Err => "btn-err",
                            MsgType::Success => "btn-success",
                            MsgType::Confirm => "btn",
                        },
                        onclick: move |_| {
                            props.state.set(PopupState::CloseWithAnimation);
                        },
                        "Ok"
                    }

                    button {
                        class: "btn-sec",
                        onclick: move |_| {
                            props.state.set(PopupState::CloseWithAnimation);
                        },
                        "Ok"
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
            replace_popup_class: "fixed sam-shadow bottom-6 right-6 bg-green-600 text-white rounded-full min-w-[400px] max-w-[90%] h-[50px] z-51",
            enter_anim_class: "animate__animated animate__slideInRight",
            leave_anim_class: "animate__animated animate__slideOutRight",
            children: rsx! {
                div { class: "flex h-[50px]",
                    div { class: "center pl-1.5", {icon!(LdCircleCheck, 40, "#00a63e", "white")} }
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

#[component]
pub fn Spinner(mut state: Signal<PopupState>) -> Element {
    rsx! {
        Popup {
            state,
            close_on_bg_click: false,
            replace_popup_class: "absolute top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] z-51",
            update_backdrop_class: "opacity-80",
            enter_anim_class: "TODO",
            leave_anim_class: "TODO",
            children: rsx! {
                div { class: "center",
                    img { src: asset!("/assets/spinner.svg") }
                }
            },
        }
    }
}
