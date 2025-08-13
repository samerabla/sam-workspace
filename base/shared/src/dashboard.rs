use crate::user::UserRole;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashNavItem {
    pub name: String,
    pub icon: String,
    pub route: String,
    pub required_roles: Vec<UserRole>,
}

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct DashNavItemInfo {
    pub name: String,
    pub icon: String,
    pub route: String,
}

impl From<DashNavItem> for DashNavItemInfo {
    fn from(item: DashNavItem) -> Self {
        Self {
            name: item.name,
            icon: item.icon,
            route: item.route,
        }
    }
}

impl DashNavItem {
    pub fn new(name: String, icon: String, route: String, required_roles: Vec<UserRole>) -> Self {
        Self {
            name,
            icon,
            route,
            required_roles,
        }
    }
}
