use std::process::id;

use dioxus::{
    html::{
        div,
        g::{format, x},
    },
    prelude::*,
};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum InputAppearance {
    #[default]
    rounded,
    square,
    line,
}

impl InputAppearance {
    pub fn class(&self) -> &'static str {
        match self {
            InputAppearance::rounded => "input-border input-rounded",
            InputAppearance::square => "input-border input-square",
            InputAppearance::line => "input-line",
        }
    }

    // pub fn label_class(&self, animated_label: bool) -> &'static str {
    //     let class = if animated_label {
    //                 "animated-label"
    //             } else {
    //                 ""
    //             };
    //     match self {
    //         InputAppearance::rounded => "animated-label",
    //         InputAppearance::square => {
    //             if animated_label {
    //                 "animated-label"
    //             } else {
    //                 ""
    //             }
    //         }
    //         InputAppearance::line => {
    //             if animated_label {
    //                 "animated-label"
    //             } else {
    //                 ""
    //             }
    //         }
    //     }
    // }
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct InputConfig {
    #[props(extends = GlobalAttributes, extends = input)]
    attributes: Vec<Attribute>,
    oninput: EventHandler<FormEvent>,
    #[props(default)]
    appearance: InputAppearance,
    #[props(default = true)]
    label: Option<String>,
    #[props(default = true)]
    animated_label: bool,
    placeholder: Option<String>,
    class: Option<String>,
    prefix: Option<Element>,
    suffix: Option<Element>,
    height: Option<u8>,
}

pub fn Input(props: InputConfig) -> Element {
    let id = use_memo(move || sam_util::gen_id!(4, "input_"));

    let class = if let Some(class) = props.class {
        format!("{} {}", props.appearance.class(), class)
    } else {
        props.appearance.class().to_string()
    };

    let height = if let Some(h) = props.height {
        format!("height: {h}px;")
    } else {
        "height: 40px;".to_string()
    };

    let mut label_padding = "";
    let mut input_padding = "";

    if let Some(_) = props.prefix {
        if props.animated_label {
            label_padding = "label-padding";
        }
        input_padding = "input-padding";
    };

    let label_class = if props.animated_label {
        format!("animated-label {label_padding}")
    } else {
        format!("label {label_padding}")
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/input.css") }
        div { class: "relative w-full {class}", style: "{height}",
            if let Some(prefix) = &props.prefix {
                div { class: "input-prefix", {prefix} }
            }
            input {
                class: "input {input_padding}",
                id: id(),
                placeholder: if props.animated_label { " " } else { props.placeholder.as_deref().unwrap_or("") },
                oninput: move |e| {
                    props.oninput.call(e);
                },
                ..props.attributes,
            }
            if props.label.is_some() {
                label { class: label_class, r#for: id(), "{props.label.as_deref().unwrap()}" }
            }
            if let Some(suffix) = &props.suffix {
                div { class: "input-suffix", {suffix} }
            }
        }
    }
}
