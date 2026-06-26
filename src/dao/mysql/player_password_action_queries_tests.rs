use super::*;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter};
use crate::game::player::{
    PlayerDetails, PlayerLoginOutcome, PlayerProfileUpdateOutcome, PlayerRegistrationOutcome,
};

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(7, "alice", "mission", "figure");
    details
}

#[test]
fn maps_successful_login_password_outcome_to_last_login_plan() {
    let outcome = PlayerPasswordActionOutcome::Login(PlayerLoginOutcome::authenticated(
        &details(),
        "secret",
        false,
        30001,
        30001,
        Some(42),
    ));

    let plans = PlayerPasswordActionQueries::plan(&outcome, 1234);

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plans[0].sql(),
        "UPDATE users SET last_online = ? WHERE id = ?"
    );
    assert_eq!(
        plans[0].parameters(),
        &[SqlParameter::Long(1234), SqlParameter::Integer(7)]
    );
}

#[test]
fn ignores_password_outcomes_without_persistent_follow_up() {
    let outcomes = vec![
        PlayerPasswordActionOutcome::Login(PlayerLoginOutcome::failed()),
        PlayerPasswordActionOutcome::Registration(PlayerRegistrationOutcome::Created),
        PlayerPasswordActionOutcome::Registration(PlayerRegistrationOutcome::NameTaken),
        PlayerPasswordActionOutcome::ProfileUpdate(PlayerProfileUpdateOutcome::Ignored),
    ];

    assert!(PlayerPasswordActionQueries::plan_all(&outcomes, 1234).is_empty());
}
