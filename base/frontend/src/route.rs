use crate::{components::*, pages::*};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/dashboard-middleware")]
    DashboardMiddleware {},
    #[nest("/dashboard")]
        #[layout(DashboardPage)]
            #[route("/")]
            DashboardHomepage {},
            #[route("/categories")]
            Categories {},
            #[route("/languages")]
            Languages {},
            #[route("/fields")]
            Field {},
         #[end_layout]
    #[end_nest]
    #[route("/login")]
    LoginPage {},

}
