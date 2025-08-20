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
use sam_ui::animation::*;

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
    use sam_ui::header::*;

    // push
    let nav = use_navigator();

    let fer = Menu::new("ferial").action(move || {
        nav.push(Route::Blog { id: 25 });
    });
    let samo = Menu::new("samo")
        .action(move || {
            nav.push(Route::Blog { id: 66 });
        })
        .children(vec![fer]);
    let nest = Menu::new("nest").children(vec![samo]);
    let go = Menu::new("last go").children(vec![nest]);
    let sub_sub_menu = Menu::new("go father").children(vec![go]);
    let sub_sub_menu2 = Menu::new("Testoo").action(|| info!("2 clicked"));
    let sub_sub_menu3 = Menu::new("sub_sub_menu").action(|| info!("3 clicked"));
    let sub_menu_1 = Menu::new("Tahsinkof go runoooooo")
        // .action(|| info!("action goes here...."))
        .children(vec![sub_sub_menu, sub_sub_menu2, sub_sub_menu3]);

    let sub_sub_menu21 = Menu::new("11").action(|| info!("11 clicked"));
    let sub_sub_menu22 = Menu::new("12").action(|| info!("12 clicked"));
    let sub_sub_menu23 = Menu::new("13").action(|| info!("13 clicked"));
    let sub_menu_2 = Menu::new("sub_menu_1")
        // .action(|| info!("action goes here...."))
        .children(vec![sub_sub_menu21, sub_sub_menu22, sub_sub_menu23]);

    let menu1 = Menu::new("Test")
        .children(vec![sub_menu_1, sub_menu_2])
        .to_root();

    let sub_menu_4 = Menu::new("a").action(|| info!("pritoooo clicked"));
    let sub_menu_3 = Menu::new("x");
    //let menu2 = Menu::new("print").children(vec![sub_menu_4, sub_menu_3]);
    let menu2 = Menu::new("slideshow")
        .action(move || {
            nav.push(Route::Test {});
        })
        .to_root();
    let menu3 = Menu::new("Home")
        .action(move || {
            nav.push(Route::Home {});
        })
        .to_root();
    let menu4 = Menu::new("not")
        .action(move || {
            nav.push(Route::PageNotFound {
                route: vec!["vvv".to_string()],
            });
        })
        .to_root();

    let mut menu_bar = use_signal(|| vec![menu3, menu1, menu2, menu4]);
    let mut is_mobile = use_signal(|| false);

    let menu_from_util = sam_util::Menu::new(rsx!("slideshow"), 0).children(vec![
        sam_util::Menu::new(rsx!("slideshow 1"), 1).children(vec![
            sam_util::Menu::new(rsx!("slideshow ***********"), 2).action(|| info!("*******")),
            sam_util::Menu::new(rsx!("slideshow --------"), 2).action(|| info!("----")),
            sam_util::Menu::new(rsx!("slideshow ++++++"), 2).action(|| info!("+++++")),
        ]),
        sam_util::Menu::new(rsx!("slideshow 2"), 0).action(|| info!("2 util clicked")),
        sam_util::Menu::new(rsx!("slideshow 3"), 0).action(|| info!("3 util clicked")),
    ]);

    let menu_from_util2 = sam_util::Menu::new(rsx!("slideshow"), 0).children(vec![
        sam_util::Menu::new(rsx!("slideshow 1"), 1).children(vec![
            sam_util::Menu::new(rsx!("slideshow ***********"), 2).action(|| info!("*******")),
            sam_util::Menu::new(rsx!("slideshow --------"), 2).action(|| info!("----")),
            sam_util::Menu::new(rsx!("slideshow ++++++"), 2).action(|| info!("+++++")),
        ]),
        sam_util::Menu::new(rsx!("slideshow 2"), 0).action(|| info!("2 util clicked")),
        sam_util::Menu::new(rsx!("slideshow 3"), 0).action(|| info!("3 util clicked")),
    ]);

    // if IS_MOBILE() {

    //     sam_ui::header::MenuList { menu_list: menu_bar() }
    // } else {

    //     sam_ui::header::MenuBar { menu_list: menu_bar() }
    // }
    // {menu_from_util.render()}
    rsx! {
        sam_util::MenuBar { menu_list: vec![menu_from_util, menu_from_util2] }
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
