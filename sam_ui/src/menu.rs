use crate::Elem;
use dioxus::{logger::tracing::info, prelude::*};
use sam_icon::icon;

#[derive(Debug, PartialEq, Clone, Props)]
pub struct MenuItemProps {
    trigger: Element,
    action: Option<Callback<()>>,
    children: Option<Element>,
}

#[derive(Clone, Copy)]
pub(crate) struct MenuState {
    pub opened_menu: Signal<Option<String>>,
}

#[component]
pub fn Menu(children: Element, update_class: Option<String>) -> Element {
    const HEADER_CLASS: Asset = asset!("/assets/header.css");
    const MAIN_CSS: Asset = asset!("/assets/main.css");
    let mut opened_menu = use_signal(|| None);
    use_context_provider(|| MenuState { opened_menu });
    use_context_provider(|| 0 as u8);
    let u_class = update_class.unwrap_or_default();
    let menu_class = format!("menu {u_class}").trim().to_string();
    rsx! {
        document::Stylesheet { href: "{MAIN_CSS}" }
        document::Stylesheet { href: "{HEADER_CLASS}" }
        div { class: "{menu_class}", {children} }
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
pub fn MenuItem(props: MenuItemProps) -> Element {
    let id = use_memo(|| sam_util::gen_id!(5, "menu_"));
    let mut is_open = use_signal(|| false);
    let mut show_children = use_signal(|| false);
    let mut opened_menu = use_context::<MenuState>().opened_menu;

    // We get the level from the parent where the first level (0) will be sent from the MenuList
    let level = use_context::<u8>();

    // We increase the level by 1 then resend to the child
    use_context_provider(|| level + 1);

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

    let mut is_rtl = use_signal(move || sam_util::is_rtl());

    let children_clone = props.children.clone();
    let children_2 = props.children.clone();
    let children_3 = props.children.clone();
    let children_4 = props.children.clone();

    let mut set_position = move || {
        // When onmounted fires, the element might not have its final computed styles yet,
        // for this reason we set the height and width on click
        is_rtl.set(sam_util::is_rtl());
        let element = Elem::from(format!("#{}", id())).0;
        if let Some(element) = element {
            let rect = element.get_bounding_client_rect();
            width.set(rect.width());
            height.set(rect.height());
        }
    };

    let onclick = move |e: Event<MouseData>| {
        set_position();
        let has_children = children_clone.is_some();
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
            } else if let Some(action) = &props.action {
                action.call(());
                opened_menu.set(None);
                is_open.set(false);
            }
        }
    };

    let onmouseenter = move |evt: Event<MouseData>| {
        // If there is an opened menu that is already open, the hover over another menu will open it directly
        set_position();
        if level == 0 && opened_menu().is_some() {
            opened_menu.set(Some(id()));
            is_open.set(true);
            show_children.set(true);
        }
        if level > 0 && children_2.is_some() {
            show_children.set(true);
        }
    };

    let onmouseleave = move |_: Event<MouseData>| {
        if level > 0 && children_3.is_some() {
            show_children.set(false);
        }
    };

    rsx! {

        div {
            class: "menu_item_wrapper",
            onclick,
            onmouseenter,
            onmouseleave,
            div { class: "menu_item center", id: "{id()}", "level": "{level}",
                div { class: "trigger", {props.trigger} }
                div { class: "menu_item_space" }
                if level > 0 && children_4.is_some() {
                    div { class: "has_children_icon",
                        {if is_rtl() { icon!(LdChevronLeft, 15) } else { icon!(LdChevronRight, 15) }}
                    }
                }
            }
            if show_children() && props.children.is_some() {
                // If there are children, we render the MenuListView
                MenuItemList {
                    children: props.children,
                    width: width(),
                    height: height(),
                    level: level + 1,
                    is_rtl,
                }
            }
        }
    }
}

#[component]
fn MenuItemList(
    children: Option<Element>,
    width: f64,
    height: f64,
    level: u8,
    is_rtl: Signal<bool>,
) -> Element {
    let h = height;
    let right = if is_rtl() {
        if level == 1 {
            "0".to_string()
        } else if level > 1 {
            format!("{width}px")
        } else {
            "auto".to_string()
        }
    } else {
        "auto".to_string()
    };
    rsx! {
        div {
            class: "menu_item_list",
            z_index: 100,
            position: "absolute",
            top: if level == 1 { "{h}px" } else { "0" },
            left: if level > 1 && !is_rtl() { "{width}px" } else if level <= 1 { "0" } else { "auto" },
            right,
            if let Some(children) = children {
                {children}
            }
        }
    }
}
