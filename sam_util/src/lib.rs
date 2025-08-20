pub use ::rand::Rng;
pub use ::rand::{distributions::Alphanumeric, thread_rng};

mod device;
pub use device::Device;

mod random;
pub mod validators;

mod web;
pub use web::to_js_array;

mod api;
pub use api::*;

mod browser;
pub use browser::*;

#[cfg(feature = "dataset")]
mod dataset;
#[cfg(feature = "dataset")]
pub use dataset::*;

#[cfg(feature = "gmail")]
pub mod gmail;

mod menu;
pub use menu::*;
