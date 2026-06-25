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
mod tests {
    use super::*;
    use crate::runtime::RoseauStartupRuntimeError;

    #[test]
    fn exposes_game_effects_and_server_outcome() {
        let outcome = RoseauApplicationTickOutcome::new(
            [GameTickEffect::SavePlayer { user_id: 7 }],
            RoseauServerLoopOutcome::Stop {
                error: RoseauStartupRuntimeError::NotListening,
            },
        );

        assert_eq!(
            outcome.game_effects(),
            &[GameTickEffect::SavePlayer { user_id: 7 }]
        );
        assert!(!outcome.should_continue());
        assert_eq!(
            outcome.server_outcome().error(),
            Some(&RoseauStartupRuntimeError::NotListening)
        );
    }
}
