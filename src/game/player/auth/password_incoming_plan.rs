use crate::game::player::PasswordAction;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PasswordIncomingPlan;

impl PasswordIncomingPlan {
    pub fn plan(effect: &IncomingExecutionEffect) -> Vec<PasswordAction> {
        let IncomingExecutionEffect::Password(action) = effect else {
            return Vec::new();
        };

        vec![action.clone()]
    }

    pub fn plan_all(effects: &[IncomingExecutionEffect]) -> Vec<PasswordAction> {
        effects.iter().flat_map(Self::plan).collect()
    }
}

#[cfg(test)]
#[path = "password_incoming_plan_tests.rs"]
mod tests;
