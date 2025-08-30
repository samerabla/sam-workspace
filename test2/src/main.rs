use std::vec;

use components::Navbar;
use dioxus::{logger::tracing::info, prelude::*};
use dioxus_elements::h1;
use sam_util::Device;
use views::{Blog, Home};

mod components;
use components::*;
mod views;

use dioxus_sdk::utils::window::use_window_size;
use sam_ui::{animation::*, Menu, MenuItem};

pub static IS_MOBILE: GlobalSignal<bool> = Signal::global(|| Device::is_mobile()());
#[derive(Debug, Clone)]
pub struct AppState {
    is_mobile: Signal<bool>,
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Nav)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: i32 },

        #[route("/test")]
        Test {  },

        #[route("/:..route")]
        PageNotFound {
            route: Vec<String>,
        },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const SAM_CSS: Asset = asset!("/assets/sam.css");

fn main() {
    dioxus::launch(App);
    eprintln!("from eprint");
    println!("from println")
}

#[component]
fn App() -> Element {
    let window_size = use_window_size();
    use_effect(move || {
        *IS_MOBILE.write() = window_size().width <= 600;
    });
    rsx! {
        // Global app resources
        document::Stylesheet { href: "https://cdnjs.cloudflare.com/ajax/libs/animate.css/4.1.1/animate.min.css" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: SAM_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        //sam_ui::header::MenuBar { menu_list: menu_bar() }
        Router::<Route> {}
        p { "{window_size().width}" }
    }
}

#[component]
fn Nav() -> Element {
    // push
    let nav = use_navigator();

    let mut is_mobile = use_signal(|| false);

    // sam_util::to_rtl();

    rsx! {
        div {
            Menu { custom_class: "tahsin",
                MenuItem {
                    trigger: rsx! {
                        div { "Test Width" }
                    },
                    MenuItem {
                        trigger: rsx! {
                            div { style: "width:500px;height:400px;background:black;color:white;",
                                p { onclick: move |_| info!("from P"), "some thing here" }
                                p { "some thing here" }
                                p { "some thing here" }
                            }
                        },

                    }
                }
                MenuItem { trigger: rsx! { "edit" },
                    MenuItem { trigger: rsx! { "open" },
                        MenuItem { trigger: rsx! { "xxx" },
                            MenuItem {
                                trigger: rsx! { "yyyy" },
                                action: Callback::new(|_| info!("yyyy")),

                            }

                        }
                        MenuItem { trigger: rsx! { "hhhh" },
                            MenuItem {
                                trigger: rsx! { "uuuuuu" },
                                action: Callback::new(|_| info!("uuuuuu")),

                            }

                        }

                    }
                    MenuItem { trigger: rsx! { "some menu" },
                        MenuItem { trigger: rsx! { "xxx" },
                            MenuItem {
                                trigger: rsx! { "yyyy" },
                                action: Callback::new(|_| info!("yyyy")),

                            }

                        }
                        MenuItem { trigger: rsx! { "hhhh" },
                            MenuItem {
                                trigger: rsx! { "uuuuuu" },
                                action: Callback::new(|_| info!("uuuuuu")),

                            }

                        }

                    }

                    hr { style: "background:#ddd" }
                    MenuItem { trigger: rsx! { "Another Menu for something" },
                        MenuItem { trigger: rsx! { "xxx" },
                            MenuItem {
                                trigger: rsx! { "yyyy" },
                                action: Callback::new(|_| info!("yyyy")),

                            }

                        }
                        MenuItem { trigger: rsx! { "hhhh" },
                            MenuItem {
                                trigger: rsx! { "uuuuuu" },
                                action: Callback::new(|_| info!("uuuuuu")),

                            }

                        }

                    }

                    MenuItem {
                        trigger: rsx! { "Samoora" },
                        action: Callback::new(|_| info!("Samoora")),

                    }
                    MenuItem { trigger: rsx! { "Abla" } }
                }
            }
        }
        // Global app resources
        Outlet::<Route> {}
        div { "footer" }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    let mut xx = use_signal(|| "");
    rsx! {}
}

use wasm_bindgen::JsCast;
#[component]
fn Test() -> Element {
    let slides = vec![
        "https://plus.unsplash.com/premium_photo-1670044020244-9da445234413?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MXx8bnVtYmVyJTIwMHxlbnwwfHwwfHx8MA%3D%3D",
        "https://images.unsplash.com/photo-1621440318464-72633426377b?q=80&w=2067&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
        "https://images.unsplash.com/photo-1621440318431-b720cd358375?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Mnx8bnVtYmVyJTIwMnxlbnwwfHwwfHx8MA%3D%3D",
        "https://images.unsplash.com/photo-1621440318357-3e3c94221a1c?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8NHx8bnVtYmVyJTIwM3xlbnwwfHwwfHx8MA%3D%3D"
    ];
    use sam_ui::slideshow::Slideshow;
    rsx! {
        h1 { "data-vvv": "fff", "Slideshow" }
        {
            Slideshow::new(slides)
                .enter("animate__slideInLeft")
                .enter_back("animate__slideInRight")
                .set_current_fixed()
                .slide_duration(1000)
                .anim_duration(1000)
                .render()
        }
    }
}

fn Samo() -> Element {
    rsx! {
        h1 { "Samo start" }
        Outlet::<Route> {}
        h1 { "Samo End" }
    }
}
