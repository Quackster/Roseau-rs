use super::roseau_application_tick_outcome::*;
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
