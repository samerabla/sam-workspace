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
pub(crate) struct MenuState {
    pub opened_menu: Signal<Option<String>>,
}

#[component]
pub fn DropdownMenu(menu: Menu) -> Element {
    rsx!(MenuBar {
        menu_list: vec![menu]
    })
}

#[component]
pub fn MenuBar(menu_list: Vec<Menu>) -> Element {
    const HEADER_CLASS: Asset = asset!("/assets/header.css");
    const MAIN_CSS: Asset = asset!("/assets/main.css");
    let mut opened_menu = use_signal(|| None);
    use_context_provider(|| MenuState { opened_menu });

    rsx! {
        document::Stylesheet { href: "{MAIN_CSS}" }
        document::Stylesheet { href: "{HEADER_CLASS}" }
        div { class: "menu_bar",
            for menu in menu_list {
                {menu.render()}
            }
        }
        if opened_menu().is_some() {
            div {
                class: "dropback",
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
    let mut is_open = use_signal(|| false);
    let mut show_children = use_signal(|| false);
    let mut opened_menu = use_context::<MenuState>().opened_menu;

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

    let menu_list_clone = menu_list.clone();
    let menu_list_2 = menu_list.clone();
    let menu_list_3 = menu_list.clone();

    let onclick = move |e: Event<MouseData>| {
        let has_children = menu_list_clone.is_some();
        let is_submenu = level > 0;

        // Handle submenu behavior
        if is_submenu && has_children {
            e.stop_propagation();
            return;
        }

        // Toggle menu state
        if is_open() {
            if !is_submenu || !has_children {
                opened_menu.set(None);
                is_open.set(false);
            }
        } else {
            opened_menu.set(Some(id()));
            is_open.set(true);

            if has_children {
                show_children.set(true);
            } else if let Some(action) = &action {
                action.call();
                opened_menu.set(None);
                is_open.set(false);
            }
        }
    };

    let onmouseenter = move |_: Event<MouseData>| {
        // // If there is an opened menu that is already open, the hover over another menu will open it directly
        // if level == 0 && opened_menu().is_some() {
        //     info!("fat 2222");
        //     opened_menu.set(Some(id()));
        //     is_open.set(true);
        //     show_children.set(true);
        // }
        if level > 0 && menu_list_2.is_some() {
            show_children.set(true);
        }
    };

    let onmouseleave = move |_: Event<MouseData>| {
        if level > 0 && menu_list_3.is_some() {
            show_children.set(false);
        }
    };

    rsx! {
        document::Stylesheet { href: "{MAIN_CSS}" }
        document::Stylesheet { href: "{HEADER_CLASS}" }

        div {
            class: "menu_wrapper",
            onclick,
            onmouseenter,
            onmouseleave,
            div {
                class: "menu center",
                id: "{id()}",
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
                    menu_list: (*menu_list).clone().unwrap(),
                    width: width(),
                    height: height(),
                }
            }
        }
    }
}

#[component]
fn MenuListView(menu_list: Vec<Menu>, width: f64, height: f64) -> Element {
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
