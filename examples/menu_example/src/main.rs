use dioxus::{logger::tracing::info, prelude::*};
use sam_ui::{Menu, MenuItem};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        h1 { "Home" }
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            // Navigation links
            Link { to: Route::Blog { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::Blog { id: id + 1 }, "Next" }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    let nav = use_navigator();

    let reload_current_page = move |_: ()| {
        // This will completely reload the page (including all assets)
        if let Some(web_window) = web_sys::window() {
            let _ = web_window.location().reload();
        }
    };

    // We can create a mega menu like this:
    let mega_menu = rsx!(MenuItem {
        trigger: rsx! {
            div { style: "width:500px;height:400px;background:black;color:white;",
                p { onclick: move |_| info!("from P"), "some thing here" }
                p { "some thing here" }
                p { "some thing here" }
            }
        },
    });

    // Custom Style:
    // We can style the menu by adding a class to custom_class property and inside it we can access all related classes
    // like menu and menu_wrapper and menu_item and so on.

    rsx! {
        div {
            Menu { custom_class: "custom_class",
                MenuItem {
                    trigger: rsx! {
                        div { class: "home",
                            {sam_icon::icon!(LdHome, 15, "red")}
                            div { style: "margin-inline-start:5px", "Home" }
                        }
                    },
                    action: Callback::new(move |_| {
                        nav.push("/");
                    }),

                }
                MenuItem {
                    trigger: rsx! {
                        div { "Mega" }
                    },
                    {mega_menu}
                }
                MenuItem { trigger: rsx! { "edit" },
                    MenuItem { trigger: rsx! { "open" },
                        MenuItem { trigger: rsx! { "blogs" },
                            MenuItem {
                                trigger: rsx! { "blog 1" },
                                action: Callback::new(move |_| {
                                    nav.push(Route::Blog { id: 1 });
                                }),

                            }
                            div { "*********" }
                            MenuItem {
                                trigger: rsx! { "blog 2" },
                                action: Callback::new(move |_| {
                                    nav.push(Route::Blog { id: 2 });
                                }),

                            }

                        }
                        MenuItem { trigger: rsx! { "hhhh" },
                            MenuItem {
                                trigger: rsx! { "uuuuuu" },
                                action: Callback::new(|_| info!("uuuuuu")),

                            }
                            MenuItem {
                                trigger: rsx! { "fff" },
                                action: Callback::new(|_| info!("uuuuuu")),

                            }
                            MenuItem {
                                trigger: rsx! { "resh" },
                                action: Callback::new(|_| info!("uuuuuu")),

                            }

                        }

                    }
                    MenuItem { trigger: rsx! { "Change Direction" },
                        MenuItem {
                            trigger: rsx! { "rtl" },
                            action: Callback::new(|_| sam_util::to_rtl()),

                        }
                        MenuItem {
                            trigger: rsx! { "ltr" },
                            action: Callback::new(|_| sam_util::to_ltr()),

                        }

                    }

                    hr { style: "background:#ddd;margin:0;" }
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
                MenuItem {
                    trigger: rsx! { "Reload Page" },
                    action: Callback::new(reload_current_page),
                }
            }
        }
        Outlet::<Route> {}
    }
}
