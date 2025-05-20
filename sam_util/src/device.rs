use dioxus_sdk::utils::window::use_window_size;

pub struct Device;

impl Device {
    pub fn is_mobile() -> impl Fn() -> bool {
        let window_size = use_window_size();

        move || {
            let width = window_size().width;
            width <= 600
        }
    }
}
