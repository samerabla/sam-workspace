use crate::{components::*, pages::*};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    HomePage {},

    #[nest("/dashboard")]
        #[route("/")]
        DashboardPage {},
        #[route("/categories")]
        Categories {},
        #[route("/languages")]
        AddLanguage {},
        #[route("/fields")]
        Field {},
    #[end_nest]

    #[route("/login")]
    LoginPage {},

}
