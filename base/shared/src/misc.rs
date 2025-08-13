use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct DashboardNavItem {
    name: String,
    resource: String,
    icon: String,
    route: String,
}
