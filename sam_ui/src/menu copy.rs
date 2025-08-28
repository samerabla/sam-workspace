use dioxus::{logger::tracing::info, prelude::*};

// TODO:
// add menu separator

// #[derive(Debug, Clone)]
// pub struct MenuBuilder {
//     trigger: Element,
//     action: Option<Callback>,
//     children: Option<Vec<MenuBuilder>>,
//     level: u8,
// }

// impl MenuBuilder {
//     pub fn new(label: Option<&str>, level: u8) -> Self {
//         Self {
//             trigger: rsx!(if let Some(label) = label {
//                 "{label}"
//             }),
//             action: None,
//             children: None,
//             level,
//         }
//     }

//     pub fn trigger(mut self, trigger: Element) -> Self {
//         self.trigger = trigger;
//         self
//     }

//     pub fn action(mut self, action: Callback) -> Self {
//         self.action = Some(action);
//         self
//     }

//     pub fn children(mut self, children: Vec<MenuBuilder>) -> Self {
//         self.children = Some(children);
//         self
//     }

//     pub fn render(self) -> Element {
//         let childrenView = rsx!(if let Some(children) = self.children {
//             for menu in children {
//                 {
//                     menu.render()
//                 }
//             }
//         });
//         let props = MenuProps {
//             trigger: self.trigger,
//             action: self.action,
//             children: if self.children.is_some() {
//                 Some(childrenView)
//             } else {
//                 None
//             },
//             level: self.level,
//         };
//         rsx! {
//             if self.level == 0 {
//                 MenuBar {
//                     Menu { trigger: self.trigger, action: self.action,
//                         if let Some(children) = self.children {
//                             for menu in children {
//                                 {menu.render()}
//                             }
//                         }
//                     }
//                 }
//             } else {
//                 Menu {
//                     trigger: self.trigger,
//                     action: if let Some(action) = self.action { Some(action) } else { None },
//                     level: self.level,
//                     if let Some(children) = self.children {
//                         for menu in children {
//                             {menu.render()}
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

#[derive(Debug, PartialEq, Clone, Props)]
pub struct MenuProps {
    trigger: Element,
    action: Option<Callback<()>>,
    children: Option<Element>,

    #[props(default)]
    level: u8,
}

// impl MenuProps {
//     pub fn new(
//         trigger: Element,
//         level: u8,
//         action: Option<Callback<()>>,
//         children: Option<Element>,
//     ) -> Self {
//         Self {
//             trigger,
//             action,
//             children,
//             level,
//         }
//     }
// }

#[derive(Clone, Copy)]
pub(crate) struct MenuState {
    pub opened_menu: Signal<Option<String>>,
}

#[component]
pub fn MenuList(children: Element) -> Element {
    const HEADER_CLASS: Asset = asset!("/assets/header.css");
    const MAIN_CSS: Asset = asset!("/assets/main.css");
    let mut opened_menu = use_signal(|| None);
    use_context_provider(|| MenuState { opened_menu });
    use_context_provider(|| 0 as u8);

    rsx! {
        document::Stylesheet { href: "{MAIN_CSS}" }
        document::Stylesheet { href: "{HEADER_CLASS}" }
        div { class: "menu_bar", {children} }
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
pub fn Menu(mut props: MenuProps) -> Element {
    const HEADER_CLASS: Asset = asset!("/assets/header.css");
    const MAIN_CSS: Asset = asset!("/assets/main.css");
    let id = use_memo(|| sam_util::gen_id!(5, "menu_"));
    let mut is_open = use_signal(|| false);
    let mut show_children = use_signal(|| false);
    let mut opened_menu = use_context::<MenuState>().opened_menu;

    // We get the level from the parent where the first level (0) will be sent from the MenuList
    props.level = use_context::<u8>();
    let level_clone = props.level.clone();

    // We increase the level by 1 then resend to the child
    use_context_provider(|| level_clone + 1);

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

    let children_clone = props.children.clone();
    let children_2 = props.children.clone();
    let children_3 = props.children.clone();

    let onclick = move |e: Event<MouseData>| {
        let has_children = children_clone.is_some();
        let is_submenu = props.level > 0;

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

    let onmouseenter = move |_: Event<MouseData>| {
        // // If there is an opened menu that is already open, the hover over another menu will open it directly
        // if level == 0 && opened_menu().is_some() {
        //     info!("fat 2222");
        //     opened_menu.set(Some(id()));
        //     is_open.set(true);
        //     show_children.set(true);
        // }
        if props.level > 0 && children_2.is_some() {
            show_children.set(true);
        }
    };

    let onmouseleave = move |_: Event<MouseData>| {
        if props.level > 0 && children_3.is_some() {
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
                {props.trigger}
            }
            if show_children() && props.children.is_some() {
                // If there are children, we render the MenuListView
                MenuListView {
                    children: props.children,
                    width: width(),
                    height: height(),
                    level: props.level + 1,
                }
            }
        }
    }
}

#[component]
fn MenuListView(children: Option<Element>, width: f64, height: f64, level: u8) -> Element {
    let h = height + 40.0;
    rsx! {
        div {
            class: "menu_list_view",
            z_index: 100,
            position: "absolute",
            top: if level == 1 { "{h}px" } else { "0" },
            left: if level > 1 { "{width}px" } else { "0" },
            if let Some(children) = children {
                {children}
            }
        }
    }
}
