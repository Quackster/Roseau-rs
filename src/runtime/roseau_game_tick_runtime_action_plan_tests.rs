use super::roseau_game_tick_runtime_action_plan::*;
use crate::game::player::{PlayerDetails, PlayerSession};

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

#[test]
fn plans_host_resolution_and_afk_connection_closes() {
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(10, 37120, details(7, "alice")));
    manager.insert(PlayerSession::new(11, 37121, details(7, "alice")));
    manager.insert(PlayerSession::new(12, 37120, details(8, "bob")));

    assert_eq!(
        RoseauGameTickRuntimeActionPlan::collect(
            &[
                GameTickRuntimeEffect::ResolveServerIp,
                GameTickRuntimeEffect::KickAfkUser { user_id: 7 },
            ],
            "roseau.local",
            &manager,
        ),
        vec![
            RoseauGameTickRuntimeActionPlan::ResolveConfiguredHost {
                host: "roseau.local".to_owned(),
            },
            RoseauGameTickRuntimeActionPlan::Network(PlayerNetworkEffect::CloseConnection {
                connection_id: 10,
            }),
            RoseauGameTickRuntimeActionPlan::Network(PlayerNetworkEffect::CloseConnection {
                connection_id: 11,
            }),
        ]
    );
}

#[test]
fn plans_credit_balance_packets_for_matching_sessions() {
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(10, 37120, details(7, "alice")));
    manager.insert(PlayerSession::new(11, 37121, details(7, "alice")));
    manager.insert(PlayerSession::new(12, 37120, details(8, "bob")));

    assert_eq!(
        RoseauGameTickRuntimeActionPlan::plan(
            &GameTickRuntimeEffect::SendCreditBalance {
                user_id: 7,
                new_balance: 125,
            },
            "roseau.local",
            &manager,
        ),
        vec![
            RoseauGameTickRuntimeActionPlan::SyncPlayerCredits {
                user_id: 7,
                credits: 125,
            },
            RoseauGameTickRuntimeActionPlan::Network(PlayerNetworkEffect::WriteResponse {
                connection_id: 10,
                packet: "#WALLETBALANCE\r125##".to_owned(),
            }),
            RoseauGameTickRuntimeActionPlan::Network(PlayerNetworkEffect::WriteResponse {
                connection_id: 11,
                packet: "#WALLETBALANCE\r125##".to_owned(),
            }),
        ]
    );
}

#[test]
fn returns_no_close_actions_for_missing_user() {
    let manager = PlayerManager::new(vec![]);

    assert!(RoseauGameTickRuntimeActionPlan::plan(
        &GameTickRuntimeEffect::KickAfkUser { user_id: 7 },
        "roseau.local",
        &manager,
    )
    .is_empty());
}
