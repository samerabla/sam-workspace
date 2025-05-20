use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    let mut x = use_signal(|| 0);
    rsx! {
        h1 { "{x}" }
        button {
            onclick: move |_| {
                spawn(async move {
                    for i in 0..10 {
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        x.set(i);
                    }
                });
                x.set(444);
            },
            "Print hello in one second"
        }
    }
}
