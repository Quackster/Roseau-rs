use super::player_password_action_report::*;
use crate::game::player::{
    PlayerDetails, PlayerLoginOutcome, PlayerProfileUpdateOutcome, PlayerRegistrationOutcome,
};

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(7, "alice", "mission", "figure");
    details
}

#[test]
fn derives_network_and_player_effects_from_password_action_outcomes() {
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

    let report = PlayerPasswordActionReport::from_outcomes(outcomes.clone(), 11);

    assert_eq!(report.outcomes(), outcomes.as_slice());
    assert_eq!(
        report.network_effects(),
        &[PlayerNetworkEffect::WriteResponse {
            connection_id: 11,
            packet: "#OK##".to_owned(),
        }]
    );
    assert_eq!(
        report.player_effects(),
        &[
            PlayerEffect::CloseConnection { connection_id: 42 },
            PlayerEffect::UpdateLastLogin { user_id: 7 },
        ]
    );
}

#[test]
fn failed_login_report_contains_error_packet_without_player_effects() {
    let report = PlayerPasswordActionReport::from_outcomes(
        [PlayerPasswordActionOutcome::Login(
            PlayerLoginOutcome::failed(),
        )],
        11,
    );

    assert_eq!(
        report.network_effects(),
        &[PlayerNetworkEffect::WriteResponse {
            connection_id: 11,
            packet: "#ERROR Login incorrect##".to_owned(),
        }]
    );
    assert!(report.player_effects().is_empty());
}
