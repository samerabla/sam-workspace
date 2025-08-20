use dioxus::prelude::*;

use super::menu::Menu;

#[component]
pub fn DropdownMenu(menu: Menu) -> Element {
    const HEADER_CLASS: Asset = asset!("/assets/header.css");
    const MAIN_CSS: Asset = asset!("/assets/main.css");

    rsx! {
        document::Stylesheet { href: "{MAIN_CSS}" }
        document::Stylesheet { href: "{HEADER_CLASS}" }
        {menu.render()}
    }
}
