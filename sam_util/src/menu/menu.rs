use std::{rc::Rc, u8};

use super::Action;
use dioxus::{logger::tracing::info, prelude::*};
use wasm_bindgen::JsCast;

// TODO:
// add menu separator

#[derive(Debug, Clone, PartialEq)]
pub struct Menu {
    label: Element,
    action: Option<Action>,
    children: Option<Vec<Menu>>,
    level: u8,
}

impl Menu {
    pub fn new(label: Element, level: u8) -> Self {
        Self {
            label,
            action: None,
            children: None,
            level,
        }
    }

    pub fn action<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.action = Some(Action::new(f));
        self
    }

    pub fn children(mut self, sub_menu_list: Vec<Menu>) -> Self {
        self.children = Some(sub_menu_list);
        self
    }

    pub fn is_root(&self) -> bool {
        self.level == 0
    }

    pub fn render(self) -> Element {
        rsx! {
            MenuView {
                label: self.label,
                action: self.action,
                menu_list: Rc::new(self.children),
                level: self.level,
            }
        }
    }
}

#[derive(Clone, Copy)]
struct MenuState {
    pub show: Signal<bool>,
}

#[derive(Clone, Copy)]
pub(crate) struct MenuBarState {
    pub opened_menu: Signal<Option<String>>,
}

#[component]
pub fn MenuBar(menu_list: Vec<Menu>) -> Element {
    const HEADER_CLASS: Asset = asset!("/assets/header.css");
    const MAIN_CSS: Asset = asset!("/assets/main.css");
    let mut opened_menu = use_signal(|| None);
    use_context_provider(|| MenuBarState { opened_menu });

    rsx! {
        document::Stylesheet { href: "{MAIN_CSS}" }
        document::Stylesheet { href: "{HEADER_CLASS}" }
        div { class: "menu_bar", z_index: 100, position: "relative",
            for menu in menu_list {
                {menu.render()}
            }
        }
        if opened_menu().is_some() {
            div {
                class: "dropback",
                z_index: 99,
                onclick: move |e: Event<MouseData>| {
                    opened_menu.set(None);
                },
            }
        }
    }
}

#[component]
fn MenuView(
    label: Element,
    menu_list: Rc<Option<Vec<Menu>>>,
    action: Option<Action>,
    level: u8,
) -> Element {
    const HEADER_CLASS: Asset = asset!("/assets/header.css");
    const MAIN_CSS: Asset = asset!("/assets/main.css");
    let id = use_memo(|| crate::gen_id!(5, "menu_"));
    // let id = crate::gen_id!(5, "menu_");
    // let id_clone = id.clone();
    // let id_clone_1 = id.clone();
    let mut is_open = use_signal(|| false);
    let mut show_children = use_signal(|| false);
    let menu_list_1 = menu_list.clone();
    let menu_list_2 = menu_list.clone();
    let mut opened_menu = use_context::<MenuBarState>().opened_menu;

    use_effect(move || {
        if opened_menu().is_some() && opened_menu() == Some(id()) {
            is_open.set(true);
            show_children.set(true);
        } else {
            is_open.set(false);
            show_children.set(false);
        }
    });

    let mut width = use_signal(|| 0.0);
    let mut height = use_signal(|| 0.0);
    // height.set(elem.as_web_event().get_bounding_client_rect().height());

    // if level == 0 {
    //                 div {
    //                     class: "dropback",
    //                     z_index: 99,
    //                     onclick: move |e: Event<MouseData>| {
    //                         e.stop_propagation();
    //                         show_children.set(false);
    //                         is_open.set(false);
    //                     },
    //                 }
    //             }

    rsx! {
        document::Stylesheet { href: "{MAIN_CSS}" }
        document::Stylesheet { href: "{HEADER_CLASS}" }

        div {
            class: "menu_wrapper",
            z_index: 100,
            onclick: move |e: Event<MouseData>| {
                if is_open() {
                    if level != 0 && menu_list_1.is_some() {
                        opened_menu.set(Some(id()));
                        return;
                    } else {
                        opened_menu.set(None);
                    }
                } else {
                    opened_menu.set(Some(id()));
                }
                is_open.toggle();
                if is_open() {
                    if menu_list_1.is_some() {
                        show_children.set(true);
                    } else {
                        if let Some(action) = &action {
                            action.call();
                            show_children.set(false);
                            opened_menu.set(None);
                        }
                    }
                } else {
                    show_children.set(false);
                }
                if level != 0 && menu_list_1.is_some() {
                    e.prevent_default();
                    is_open.toggle();
                    show_children.set(true);
                }
            },
            onmouseenter: move |_| {
                if level > 0 {
                    show_children.set(true);
                }
            },
            onmouseleave: move |_| {
                if level > 0 {
                    show_children.set(false);
                }
            },
            div {
                class: "menu center",
                id: "{id()}",
                z_index: 100,
                onmounted: move |elem: Event<MountedData>| async move {
                    use dioxus_web::WebEventExt;
                    width.set(elem.as_web_event().get_bounding_client_rect().width());
                    height.set(elem.as_web_event().get_bounding_client_rect().height());
                },
                {label}
            }
            if show_children() && menu_list.is_some() {
                // If there are children, we render the MenuListView
                MenuListView {
                    show: show_children,
                    menu_list: (*menu_list).clone().unwrap(),
                    width: width(),
                    height: height(),
                }
            }
        }
    }
}

#[component]
fn MenuListView(show: Signal<bool>, menu_list: Vec<Menu>, width: f64, height: f64) -> Element {
    let level = menu_list.first().map(|m| m.level).unwrap_or_default();
    let h = height + 40.0;
    rsx! {
        div {
            class: "menu_list_view",
            z_index: 100,
            position: "absolute",
            top: if level == 1 { "{h}px" } else { "0" },
            left: if level > 1 { "{width}px" } else { "0" },
            for menu in menu_list {
                {menu.render()}
            }
        }
    }
}
