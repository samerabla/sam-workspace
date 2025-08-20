use std::{rc::Rc, u8};

use super::Action;
use dioxus::{
    html::{menu, u::is},
    logger::tracing::info,
    prelude::*,
};

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

#[component]
fn MenuView(
    label: Element,
    menu_list: Rc<Option<Vec<Menu>>>,
    action: Option<Action>,
    level: u8,
) -> Element {
    const HEADER_CLASS: Asset = asset!("/assets/header.css");
    const MAIN_CSS: Asset = asset!("/assets/main.css");

    let id = crate::gen_id!(5, "menu_");
    let mut is_open = use_signal(|| false);
    let mut show_children = use_signal(|| false);
    let menu_list_1 = menu_list.clone();
    let menu_list_2 = menu_list.clone();
    // let click_handler = move |e: Event<MouseData>| {
    //     //e.stop_propagation();
    //     // Check if the menu is opened
    //     is_open.toggle();
    // };
    use_effect(move || {
        // if is_open() {
        //     if menu_list_1.is_some() {
        //         show_children.set(true);
        //     } else {
        //         if let Some(action) = &action {
        //             action.call();

        //             show_children.set(false);
        //         }
        //     }
        // } else {
        //     show_children.set(false);
        // }
    });

    let mut width = use_signal(|| 0.0);
    let mut height = use_signal(|| 0.0);

    /**
    onclick: move |e: Event<MouseData>| {
               is_open.toggle();
               if is_open() {
                   if menu_list_1.is_some() {
                       show_children.set(true);
                   } else {
                       if let Some(action) = &action {
                           action.call();
                           show_children.set(false);
                       }
                   }
               } else {
                   show_children.set(false);
               }
               if level != 0 && menu_list_1.is_some() {
                   info!("Menu clicked, level: {level}, is_open: {is_open}");
                   e.stop_propagation();
                   is_open.toggle();
                   show_children.set(true);
               }
           },
    */

    rsx! {
        document::Stylesheet { href: "{MAIN_CSS}" }
        document::Stylesheet { href: "{HEADER_CLASS}" }

        div {
            class: "menu_wrapper",
            onclick: move |e: Event<MouseData>| {
                is_open.toggle();
                if is_open() {
                    if menu_list_1.is_some() {
                        show_children.set(true);
                    } else {
                        if let Some(action) = &action {
                            action.call();
                            show_children.set(false);
                        }
                    }
                } else {
                    show_children.set(false);
                }
                if level != 0 && menu_list_1.is_some() {
                    info!("Menu clicked, level: {level}, is_open: {is_open}");
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
                id: "{id}",
                z_index: 100 - level,
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
                if level == 0 {

                    div {
                        class: "dropback",
                        z_index: 100 - level - 1,
                        onclick: move |e: Event<MouseData>| {
                            e.stop_propagation();
                            show_children.set(false);
                            is_open.set(false);
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn MenuListView(show: Signal<bool>, menu_list: Vec<Menu>, width: f64, height: f64) -> Element {
    let level = menu_list.first().map(|m| m.level).unwrap_or_default();
    rsx! {
        div {
            z_index: 100 + level,
            position: "absolute",
            top: if level == 1 { "{height}px" } else { "0" },
            left: if level > 1 { "{width}px" } else { "0" },
            for menu in menu_list {
                {menu.render()}
            }
        }
    }
}
