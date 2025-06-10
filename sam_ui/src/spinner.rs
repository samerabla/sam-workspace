#[allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum SpinnerVariant {
    #[default]
    Moon,
    FadeDots,
}

impl SpinnerVariant {
    pub fn render(&self, config: &SpinnerConfig) -> Element {
        match self {
            SpinnerVariant::Moon => rsx!({ Moon(config.clone()) }),
            SpinnerVariant::FadeDots => rsx!({ FadeDots(config.clone()) }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct SpinnerConfig {
    size: Option<u16>,
    #[props(default = "var(--color-primary-500)".to_string())]
    color: String,
    #[props(default = "1s".to_string())]
    duration: String,
    #[props(default)]
    variant: SpinnerVariant,
}

#[component]
pub fn Spinner(props: SpinnerConfig) -> Element {
    rsx! {
        {props.variant.render(&props)}
    }
}

#[component]
pub fn Moon(props: SpinnerConfig) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 100 100",
            preserve_aspect_ratio: "xMidYMid",
            style: "shape-rendering: auto; display: block; background: rgb(255, 255, 255);",
            width: props.size.unwrap_or(100).to_string(),
            height: props.size.unwrap_or(100).to_string(),
            g {
                path {
                    style: "fill:{props.color};stroke:none;",
                    d: "M10 50A40 40 0 0 0 90 50A40 42 0 0 1 10 50",
                    animateTransform {
                        values: "0 50 51;360 50 51",
                        key_times: "0;1",
                        repeat_count: "indefinite",
                        dur: props.duration,
                        r#type: "rotate",
                        attribute_name: "transform",
                    }
                }
                g {}
            }
        }
    }
}

#[component]
pub fn FadeDots(props: SpinnerConfig) -> Element {
    let size_2 = (props.size.unwrap_or(100) + props.size.unwrap_or(100) * 3).to_string();
    let size_3 = (props.size.unwrap_or(100) + props.size.unwrap_or(100) * 6).to_string();

    rsx!(
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 200 200",
            rect {
                style: "fill:{props.color};stroke:{props.color};",
                stroke_width: props.size.unwrap_or(100).to_string(),
                width: props.size.unwrap_or(100).to_string(),
                height: props.size.unwrap_or(100).to_string(),
                x: props.size.unwrap_or(100).to_string(),
                y: props.size.unwrap_or(100).to_string(),
                animate {
                    attribute_name: "opacity",
                    calc_mode: "spline",
                    dur: "2",
                    values: "1;0;1;",
                    key_splines: ".5 0 .5 1;.5 0 .5 1",
                    repeat_count: "indefinite",
                    begin: "-.4",
                }
            }
            rect {
                style: "fill:{props.color};stroke:{props.color};",
                stroke_width: props.size.unwrap_or(100).to_string(),
                width: props.size.unwrap_or(100).to_string(),
                height: props.size.unwrap_or(100).to_string(),
                x: size_2,
                y: props.size.unwrap_or(100).to_string(),
                animate {
                    attribute_name: "opacity",
                    calc_mode: "spline",
                    dur: "2",
                    values: "1;0;1;",
                    key_splines: ".5 0 .5 1;.5 0 .5 1",
                    repeat_count: "indefinite",
                    begin: "-.2",
                }
            }
            rect {
                style: "fill:{props.color};stroke:{props.color};",
                stroke_width: props.size.unwrap_or(100).to_string(),
                width: props.size.unwrap_or(100).to_string(),
                height: props.size.unwrap_or(100).to_string(),
                x: size_3,
                y: props.size.unwrap_or(100).to_string(),
                animate {
                    attribute_name: "opacity",
                    calc_mode: "spline",
                    dur: "2",
                    values: "1;0;1;",
                    key_splines: ".5 0 .5 1;.5 0 .5 1",
                    repeat_count: "indefinite",
                    begin: "0",
                }
            }
        }
    )
}
