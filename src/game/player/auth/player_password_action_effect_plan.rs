use crate::game::player::{PlayerEffect, PlayerPasswordActionOutcome};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerPasswordActionEffectPlan;

impl PlayerPasswordActionEffectPlan {
    pub fn plan(outcome: &PlayerPasswordActionOutcome) -> Vec<PlayerEffect> {
        match outcome {
            PlayerPasswordActionOutcome::Login(login) => login.effects().to_vec(),
            PlayerPasswordActionOutcome::Registration(_)
            | PlayerPasswordActionOutcome::ProfileUpdate(_) => Vec::new(),
        }
    }

    pub fn plan_all(outcomes: &[PlayerPasswordActionOutcome]) -> Vec<PlayerEffect> {
        outcomes.iter().flat_map(Self::plan).collect()
    }
}

#[cfg(test)]
#[path = "player_password_action_effect_plan_tests.rs"]
mod tests;
