use super::*;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter};
use crate::game::player::{
    PlayerDetails, PlayerEffect, PlayerLoginOutcome, PlayerRegistrationOutcome,
};
use crate::server::PlayerNetworkEffect;

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(7, "alice", "mission", "figure");
    details
}

#[test]
fn carries_player_report_and_mysql_persistence_plans() {
    let report = MySqlPlayerPasswordActionReport::from_outcomes(
        [
            PlayerPasswordActionOutcome::Login(PlayerLoginOutcome::authenticated(
                &details(),
                "secret",
                false,
                30001,
                30001,
                Some(42),
            )),
            PlayerPasswordActionOutcome::Registration(PlayerRegistrationOutcome::Created),
        ],
        11,
        1234,
    );

    assert_eq!(
        report.player_report().network_effects(),
        &[PlayerNetworkEffect::WriteResponse {
            connection_id: 11,
            packet: "#OK##".to_owned(),
        }]
    );
    assert_eq!(
        report.player_report().player_effects(),
        &[
            PlayerEffect::CloseConnection { connection_id: 42 },
            PlayerEffect::UpdateLastLogin { user_id: 7 },
        ]
    );
    assert_eq!(report.persistence_plans().len(), 1);
    assert_eq!(
        report.persistence_plans()[0].kind(),
        SqlExecutionKind::Execute
    );
    assert_eq!(
        report.persistence_plans()[0].sql(),
        "UPDATE users SET last_online = ? WHERE id = ?"
    );
    assert_eq!(
        report.persistence_plans()[0].parameters(),
        &[SqlParameter::Long(1234), SqlParameter::Integer(7)]
    );
}

#[test]
fn failed_login_has_packet_but_no_persistence_plan() {
    let report = MySqlPlayerPasswordActionReport::from_outcomes(
        [PlayerPasswordActionOutcome::Login(
            PlayerLoginOutcome::failed(),
        )],
        11,
        1234,
    );

    assert_eq!(
        report.player_report().network_effects(),
        &[PlayerNetworkEffect::WriteResponse {
            connection_id: 11,
            packet: "#ERROR Login incorrect##".to_owned(),
        }]
    );
    assert!(report.player_report().player_effects().is_empty());
    assert!(report.persistence_plans().is_empty());
}
