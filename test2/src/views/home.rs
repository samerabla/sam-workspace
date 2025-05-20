use crate::{components::Hero, Route};
use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn Home() -> Element {
    rsx! {
        sam_ui::animation::AnimateWrapper { to: "slideInLeft", "home" }
    }
    // Build cool things ✌️
    // use sam_ui::header::*;

    // // push
    // let nav = use_navigator();

    // let fer = SubMenu::new("ferial").action(move || {
    //     nav.push(Route::Blog { id: 1 });
    // });
    // let samo = SubMenu::new("samo")
    //     .action(|| info!("go samo..."))
    //     .children(vec![fer]);
    // let nest = SubMenu::new("nest")
    //     .action(|| info!("go nest..."))
    //     .children(vec![samo]);
    // let go = SubMenu::new("last go")
    //     .action(|| info!("go Samaira..."))
    //     .children(vec![nest]);
    // let sub_sub_menu = SubMenu::new("go father")
    //     .action(|| info!("1 clicked"))
    //     .children(vec![go]);
    // let sub_sub_menu2 = SubMenu::new("Testoo").action(|| info!("2 clicked"));
    // let sub_sub_menu3 = SubMenu::new("sub_sub_menu").action(|| info!("3 clicked"));
    // let sub_menu_1 = SubMenu::new("Tahsinkof go runoooooo")
    //     // .action(|| info!("action goes here...."))
    //     .children(vec![sub_sub_menu, sub_sub_menu2, sub_sub_menu3]);

    // let sub_sub_menu21 = SubMenu::new("11").action(|| info!("11 clicked"));
    // let sub_sub_menu22 = SubMenu::new("12").action(|| info!("12 clicked"));
    // let sub_sub_menu23 = SubMenu::new("13").action(|| info!("13 clicked"));
    // let sub_menu_2 = SubMenu::new("sub_menu_1")
    //     // .action(|| info!("action goes here...."))
    //     .children(vec![sub_sub_menu21, sub_sub_menu22, sub_sub_menu23]);

    // let menu1 = Menu::new("Test").children(vec![sub_menu_1, sub_menu_2]);

    // let sub_menu_4 = SubMenu::new("a").action(|| info!("pritoooo clicked"));
    // let sub_menu_3 = SubMenu::new("x");
    // let menu2 = Menu::new("print").children(vec![sub_menu_4, sub_menu_3]);

    // let mut menu_bar = use_signal(|| vec![menu1, menu2]);

    // rsx! {
    //     // Global app resources
    //     sam_ui::header::MenuBar { menu_list: menu_bar() }
    // }
}
