use shared::user::User;

use shared::{Action, AttributeValue, Context, Resource};

pub trait PolicyEvaluator {
    fn evaluate(
        &self,
        user: &User,
        resource: &Resource,
        action: &Action,
        context: &Context,
    ) -> bool;
}

// Example policy: Users can edit their own posts
pub struct OwnershipPolicy;

impl PolicyEvaluator for OwnershipPolicy {
    fn evaluate(
        &self,
        user: &User,
        resource: &Resource,
        action: &Action,
        _context: &Context,
    ) -> bool {
        // if action.name == "edit" && resource.resource_type == "post" {
        //     if let (Some(AttributeValue::Number(user_id)), Some(AttributeValue::Number(owner_id))) = (
        //         user.attributes.get("id"),
        //         resource.attributes.get("owner_id"),
        //     ) {
        //         return user_id == owner_id;
        //     }
        // }
        false
    }
}

// Policy engine that combines multiple policies
pub struct PolicyEngine {
    policies: Vec<Box<dyn PolicyEvaluator + Send + Sync>>,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }

    pub fn add_policy<P: PolicyEvaluator + Send + Sync + 'static>(&mut self, policy: P) {
        self.policies.push(Box::new(policy));
    }

    pub fn authorize(
        &self,
        user: &User,
        resource: &Resource,
        action: &Action,
        context: &Context,
    ) -> bool {
        self.policies
            .iter()
            .all(|policy| policy.evaluate(user, resource, action, context))
    }
}
