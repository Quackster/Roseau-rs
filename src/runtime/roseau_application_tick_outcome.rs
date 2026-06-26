use crate::game::GameTickEffect;
use crate::runtime::RoseauServerLoopOutcome;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoseauApplicationTickOutcome {
    game_effects: Vec<GameTickEffect>,
    server_outcome: RoseauServerLoopOutcome,
}

impl RoseauApplicationTickOutcome {
    pub fn new(
        game_effects: impl Into<Vec<GameTickEffect>>,
        server_outcome: RoseauServerLoopOutcome,
    ) -> Self {
        Self {
            game_effects: game_effects.into(),
            server_outcome,
        }
    }

    pub fn game_effects(&self) -> &[GameTickEffect] {
        &self.game_effects
    }

    pub fn server_outcome(&self) -> &RoseauServerLoopOutcome {
        &self.server_outcome
    }

    pub fn should_continue(&self) -> bool {
        self.server_outcome.should_continue()
    }
}

#[cfg(test)]
#[path = "roseau_application_tick_outcome_tests.rs"]
mod tests;
