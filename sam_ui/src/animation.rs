use std::{cell::RefCell, rc::Rc};

use dioxus::{logger::tracing::info, prelude::*};
use dioxus_web::WebEventExt;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    Event, HtmlCollection, IntersectionObserver, IntersectionObserverEntry,
    IntersectionObserverInit,
};

use crate::element::Elem;

const CSS: Asset = asset!("/assets/slideshow.css");

pub fn animate_staggered<T>(selector: T, from: &str, to: &str)
where
    T: Into<Elem>,
{
    animate_staggered_full(selector, from, to, "", "", 50, None::<fn()>);
}

pub fn animate_staggered_full<T>(
    selector: T,
    from: &str,
    to: &str,
    from_style: &str,
    to_style: &str,
    interval: u32,
    callback: Option<impl FnMut() + 'static>,
) where
    T: Into<Elem>,
{
    if let Some(elem) = selector.into().0 {
        let child_elements: HtmlCollection = elem.children();
        let callback = callback.map(|cb| Rc::new(RefCell::new(cb)));

        for i in 0..child_elements.length() {
            if let Some(child) = child_elements.item(i) {
                let child = Rc::new(RefCell::new(child));
                let mut style = String::from("");

                // Add the initial class if exist
                if !from.is_empty() {
                    let arr = sam_util::to_js_array(from);
                    child.borrow_mut().class_list().add(&arr).ok();
                }

                // Add the animation class if exist
                if !to.is_empty() {
                    let arr = sam_util::to_js_array(to);
                    child.borrow_mut().class_list().add(&arr).ok();
                }

                // Add the delay style
                let delay_style = format!("animation-delay: {}ms;", i * interval);
                style.push_str(delay_style.as_str());

                // Add the initial style if exist
                if !from_style.is_empty() {
                    style.push_str(from_style);
                }

                // Add the animation style if exist for modifying only dynamic properties
                if !to_style.is_empty() {
                    style.push_str(to_style);
                }

                // Handle the origin style in order to not be deleted
                if let Some(original_style) = child.borrow_mut().get_attribute("style") {
                    let stl: Vec<&str> = original_style.split(&style).collect();
                    let origin = stl.get(0);
                    if let Some(origin) = origin {
                        if !origin.is_empty() {
                            let new_style = format!("{origin}{style}");
                            style.clear();
                            style.push_str(new_style.as_str());
                        }
                    }
                }

                // Add the aggregated style
                child
                    .borrow_mut()
                    .set_attribute("style", style.as_str())
                    .ok();

                let child_clone = child.clone();
                let to_owned = to.to_string(); // Convert &str to String
                let from_owned = from.to_string();
                // if let Some(func) = callback.clone() {
                // }
                // Create an event listener for 'animationend'
                let closure = Closure::wrap(Box::new({
                    let child_clone = child_clone.clone();
                    let to = to_owned.clone(); // Move owned String inside closure
                    let from = from_owned.clone();
                    let cb = callback.clone();
                    move |_: Event| {
                        //----------------------------
                        let child = child_clone.borrow_mut();
                        // Remove the added classes
                        let arr_from = sam_util::to_js_array::<&str>(from.as_ref());
                        let arr_to = sam_util::to_js_array::<&str>(to.as_ref());

                        child.class_list().remove(&arr_to).ok();
                        child.class_list().remove(&arr_from).ok();

                        // Remove the "class" attribute if it's empty (or only whitespace)
                        if let Some(class_value) = child.get_attribute("class") {
                            if class_value.trim().is_empty() {
                                child.remove_attribute("class").ok();
                            }
                        }

                        // Remove the added style
                        if let Some(style) = child.get_attribute("style") {
                            let stl: Vec<&str> = style.split("animation-delay").collect();
                            if let Some(origin) = stl.get(0) {
                                if !origin.is_empty() {
                                    child.set_attribute("style", origin).ok();
                                }
                            }
                        }

                        if let Some(func) = cb.clone() {
                            func.borrow_mut()();
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                // Attach event listener to the element
                child
                    .borrow_mut()
                    .add_event_listener_with_callback(
                        "animationend",
                        closure.as_ref().unchecked_ref(),
                    )
                    .ok();

                // Keep closure alive until event triggers
                closure.forget();
            }
        }
    }
}

pub fn animate<S>(
    target: Option<web_sys::Element>,
    animation_class: S,
    animation_style: Option<&str>,
    callback: Option<impl FnMut() + 'static>,
) where
    S: AsRef<str>,
{
    // Apply the animation class
    if let Some(elem) = target {
        // elem.class_list().add_1(animation_class).ok();
        if !animation_class.as_ref().is_empty() {
            let arr = sam_util::to_js_array(animation_class.as_ref());
            elem.class_list()
                .add(&arr)
                .expect("Failed to add animation class");
        }
        // Modify only dynamic properties
        if let Some(style) = animation_style {
            elem.set_attribute("style", style)
                .expect("Failed to set style attributes");
        }

        if let Some(mut func) = callback {
            // Create an event listener for 'animationend'
            let closure = Closure::wrap(Box::new(move |_: Event| {
                func();
            }) as Box<dyn FnMut(_)>);

            // Attach event listener to the element
            elem.add_event_listener_with_callback("animationend", closure.as_ref().unchecked_ref())
                .expect("Failed to add event listener");

            // Keep closure alive until event triggers
            closure.forget();
        }
    } else {
        if let Some(mut func) = callback {
            func();
        }
    }
}

// pub fn animate<T, S>(
//     selector: T,
//     animation_class: S,
//     animation_style: Option<&str>,
//     callback: Option<impl FnMut() + 'static>,
// ) where
//     T: Into<Elem> + Clone,
//     S: AsRef<str>,
// {
//     let sel = selector.clone();
//     info!("selector: {:?}", sel.into());
//     // Apply the animation class
//     if let Some(elem) = selector.into().0 {
//         // elem.class_list().add_1(animation_class).ok();
//         let arr = sam_util::to_js_array(animation_class.as_ref());
//         elem.class_list()
//             .add(&arr)
//             .expect("Failed to add animation class");
//         info!("class: {:?}", elem.class_list());
//         // Modify only dynamic properties
//         if let Some(style) = animation_style {
//             elem.set_attribute("style", style)
//                 .expect("Failed to set style attributes");
//         }

//         if let Some(mut func) = callback {
//             // Create an event listener for 'animationend'
//             let closure = Closure::wrap(Box::new(move |_: Event| {
//                 func();
//             }) as Box<dyn FnMut(_)>);

//             // Attach event listener to the element
//             elem.add_event_listener_with_callback("animationend", closure.as_ref().unchecked_ref())
//                 .expect("Failed to add event listener");

//             // Keep closure alive until event triggers
//             closure.forget();
//         }
//     } else {
//         if let Some(mut func) = callback {
//             func();
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationTiming {
    OnMounted,
    OnHover,
}

#[derive(PartialEq, Props, Clone)]
pub struct AnimationProps {
    #[props(default = "".to_string())]
    from: String,

    #[props(default = "".to_string())]
    to: String,

    #[props(default = "".to_string())]
    from_style: String,

    #[props(default = "".to_string())]
    to_style: String,

    #[props(default = 50)]
    interval: u32,

    children: Element,

    #[props(default = AnimationTiming::OnMounted)]
    trigger_on: AnimationTiming,
    onmounted: Option<EventHandler<web_sys::Element>>,
}

// pub fn AnimatedOnHover(props: AnimationProps) -> Element {
//     rsx! {
//         AnimationWrapper {
//             to: props.to,
//             from: props.from,
//             trigger_on: AnimationTiming::OnHover,
//             {props.children}
//         }
//     }
// }
#[component]
pub fn AnimateWrapper(props: AnimationProps) -> Element {
    let mut target = use_signal(|| None);

    let id = use_memo(|| sam_util::gen_id!());
    let id_selector = use_memo(move || format!("#{}", id()));

    let animate = Callback::new(move |()| {
        animate(
            target(),
            props.to.as_str(),
            Some(props.to_style.as_str()),
            Some({
                let to = props.to.clone();
                move || {
                    Elem::from(id_selector()).remove_class(to.as_str());
                }
            }),
        )
    });
    rsx! {
        document::Stylesheet { href: "{CSS}" }
        div {
            id: id(),
            class: "{props.from}",
            onmounted: {
                let trigger_on = props.trigger_on.clone();
                move |e: dioxus::prelude::Event<MountedData>| {
                    use dioxus_web::WebEventExt;
                    let web_sys_elem = e.as_web_event();
                    target.set(Some(web_sys_elem));
                    if let Some(evt) = props.onmounted {
                        evt.call(target().unwrap());
                    }
                    if trigger_on == AnimationTiming::OnMounted {
                        animate.call(())
                    }
                }
            },
            onmouseenter: {
                let trigger_on = props.trigger_on.clone();
                move |_| {
                    if trigger_on == AnimationTiming::OnHover {
                        animate.call(());
                    }
                }
            },
            {props.children}
        }
    }
}

// #[component]
// pub fn AnimateWrapper1(props: AnimationProps) -> Element {
//     let id = use_memo(|| sam_util::gen_id!());
//     let id_selector = use_memo(move || format!("#{}", id()));

//     let animate = Callback::new(move |()| {
//         animate(
//             id_selector(),
//             props.to.as_str(),
//             Some(props.to_style.as_str()),
//             Some({
//                 let to = props.to.clone();
//                 move || {
//                     Elem::from(id_selector()).remove_class(to.as_str());
//                 }
//             }),
//         )
//     });
//     rsx! {
//         document::Stylesheet { href: "{CSS}" }
//         div {
//             id: id(),
//             class: "{props.from}",
//             onmounted: {
//                 let trigger_on = props.trigger_on.clone();
//                 move |_| {
//                     if trigger_on == AnimationTiming::OnMounted {
//                         animate.call(())
//                     }
//                 }
//             },
//             onmouseenter: {
//                 let trigger_on = props.trigger_on.clone();
//                 move |_| {
//                     if trigger_on == AnimationTiming::OnHover {
//                         animate.call(());
//                     }
//                 }
//             },
//             {props.children}
//         }
//     }
// }

#[component]
pub fn AnimateStaggered(props: AnimationProps) -> Element {
    let id = use_memo(|| sam_util::gen_id!());
    let id_selector = use_memo(move || format!("#{}", id()));
    let animate: Callback = Callback::new(move |()| {
        animate_staggered_full(
            id_selector(),
            props.from.as_str(),
            props.to.as_str(),
            props.from_style.as_str(),
            props.to_style.as_str(),
            props.interval,
            None::<fn()>,
        );
    });
    rsx! {
        document::Stylesheet { href: "{CSS}" }
        div {
            id: id(),
            //class: "{props.from}",
            onmounted: {
                let trigger_on = props.trigger_on.clone();
                move |_| {
                    if trigger_on == AnimationTiming::OnMounted {
                        animate.call(())
                    }
                }
            },
            onmouseenter: {
                let trigger_on = props.trigger_on.clone();
                move |_| {
                    if trigger_on == AnimationTiming::OnHover {
                        animate.call(());
                    }
                }
            },
            {props.children}
        }
    }
}

#[component]
pub fn AnimateOnScroll(props: AnimationProps) -> Element {
    let mut elem: Signal<Option<web_sys::Element>> = use_signal(|| None);
    let mut is_visible = use_signal(|| false);
    let mut observer: Signal<Option<IntersectionObserver>> = use_signal(|| None);

    use_effect(move || {
        let observer_callback = Closure::wrap(Box::new(move |entries: Vec<JsValue>, _: JsValue| {
            let entry: IntersectionObserverEntry = entries[0].clone().into();
            if entry.is_intersecting() {
                is_visible.set(true);
            } else {
                is_visible.set(false);
            }
        }) as Box<dyn FnMut(Vec<JsValue>, JsValue)>);

        let observer_options = IntersectionObserverInit::new();
        // observer_options.set_threshold(&JsValue::from_f64(0.5));

        let _observer = IntersectionObserver::new_with_options(
            observer_callback.as_ref().unchecked_ref(),
            &observer_options,
        )
        .expect("Failed to create observer");

        // let _observer = IntersectionObserver::new_with_options(
        //     observer_callback.as_ref().unchecked_ref(),
        //     &IntersectionObserverInit::new(),
        // )
        // .expect("Failed to create observer");

        observer.set(Some(_observer));

        //observer.observe(&element);
        if let Some(observer) = observer() {
            if let Some(elem) = elem() {
                observer.observe(&elem);
            }
        }

        observer_callback.forget();
    });

    if is_visible() {
        animate(elem(), props.to.as_str(), None, None::<fn()>)
        // if let Some(elem) = elem() {
        // }
    } else {
        if let Some(elem) = elem() {
            let arr = sam_util::to_js_array::<&str>(props.to.as_ref());
            elem.class_list().remove(&arr).ok();
        }
    }
    rsx! {
        document::Stylesheet { href: "{CSS}" }
        div {
            class: "{props.from}",
            onmounted: move |evt: MountedEvent| {
                elem.set(Some(evt.as_web_event()));
            },
            {props.children}
        }
    }
}
