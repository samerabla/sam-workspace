use dioxus::prelude::*;
use frontend::App;

fn main() {
    dioxus::launch(App);
}

fn App1() -> Element {
    rsx!(
        h1 { "samoora" }
    )
}
