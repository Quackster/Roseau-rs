use super::*;
use crate::game::player::{
    PlayerDetails, PlayerLoginOutcome, PlayerProfileUpdateOutcome, PlayerRegistrationOutcome,
};

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(7, "alice", "mission", "figure");
    details
}

#[test]
fn extracts_login_follow_up_effects_from_password_action_outcomes() {
    let outcomes = vec![
        PlayerPasswordActionOutcome::Login(PlayerLoginOutcome::authenticated(
            &details(),
            "secret",
            false,
            30001,
            30001,
            Some(42),
        )),
        PlayerPasswordActionOutcome::Registration(PlayerRegistrationOutcome::Created),
        PlayerPasswordActionOutcome::ProfileUpdate(PlayerProfileUpdateOutcome::Ignored),
    ];

    assert_eq!(
        PlayerPasswordActionEffectPlan::plan_all(&outcomes),
        vec![
            PlayerEffect::CloseConnection { connection_id: 42 },
            PlayerEffect::UpdateLastLogin { user_id: 7 },
        ]
    );
}

#[test]
fn failed_login_and_non_login_outcomes_have_no_follow_up_effects() {
    for outcome in [
        PlayerPasswordActionOutcome::Login(PlayerLoginOutcome::failed()),
        PlayerPasswordActionOutcome::Registration(PlayerRegistrationOutcome::NameTaken),
        PlayerPasswordActionOutcome::ProfileUpdate(PlayerProfileUpdateOutcome::Ignored),
    ] {
        assert!(PlayerPasswordActionEffectPlan::plan(&outcome).is_empty());
    }
}
