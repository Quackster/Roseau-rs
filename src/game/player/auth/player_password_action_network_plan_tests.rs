use super::player_password_action_network_plan::*;
use crate::game::player::{
    PlayerLoginOutcome, PlayerProfileUpdateOutcome, PlayerRegistrationOutcome,
};

#[test]
fn plans_login_and_registration_packets_from_password_action_outcomes() {
    let outcomes = vec![
        PlayerPasswordActionOutcome::Login(PlayerLoginOutcome::failed()),
        PlayerPasswordActionOutcome::Registration(PlayerRegistrationOutcome::Created),
        PlayerPasswordActionOutcome::Registration(PlayerRegistrationOutcome::NameTaken),
    ];

    let effects = PlayerPasswordActionNetworkPlan::plan_all(&outcomes, 42);

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#ERROR Login incorrect##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#OK##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#BADNAME##".to_owned(),
            },
        ]
    );
}

#[test]
fn profile_update_outcomes_do_not_emit_direct_packets() {
    let effects = PlayerPasswordActionNetworkPlan::plan(
        &PlayerPasswordActionOutcome::ProfileUpdate(PlayerProfileUpdateOutcome::Ignored),
        42,
    );

    assert!(effects.is_empty());
}
