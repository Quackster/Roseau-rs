use super::*;
use crate::game::player::{PlayerDetails, PlayerSession};
use crate::messages::outgoing::SystemBroadcast;

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

fn player_manager() -> PlayerManager {
    let mut manager = PlayerManager::new(Vec::new());
    manager.insert(PlayerSession::new(10, 30000, details(1, "alice")));
    manager.insert(PlayerSession::new(11, 30001, details(1, "alice-room")));
    manager.insert(PlayerSession::new(20, 30000, details(2, "bob")));
    manager
}

#[test]
fn plans_alerts_and_direct_closes() {
    let manager = player_manager();

    let effects = PlayerEffectNetworkPlan::plan_all(
        &[
            PlayerEffect::SendAlert(SystemBroadcast::new("maintenance")),
            PlayerEffect::CloseConnection { connection_id: 20 },
        ],
        &manager,
    );

    assert_eq!(effects.len(), 4);
    assert!(effects.contains(&PlayerNetworkEffect::WriteResponse {
        connection_id: 10,
        packet: "#SYSTEMBROADCAST\rmaintenance##".to_owned(),
    }));
    assert!(effects.contains(&PlayerNetworkEffect::WriteResponse {
        connection_id: 11,
        packet: "#SYSTEMBROADCAST\rmaintenance##".to_owned(),
    }));
    assert!(effects.contains(&PlayerNetworkEffect::WriteResponse {
        connection_id: 20,
        packet: "#SYSTEMBROADCAST\rmaintenance##".to_owned(),
    }));
    assert!(effects.contains(&PlayerNetworkEffect::CloseConnection { connection_id: 20 }));
}

#[test]
fn closes_all_connections_for_user() {
    let manager = player_manager();

    let effects =
        PlayerEffectNetworkPlan::plan(&PlayerEffect::CloseUserConnections { user_id: 1 }, &manager);

    assert_eq!(effects.len(), 2);
    assert!(effects.contains(&PlayerNetworkEffect::CloseConnection { connection_id: 10 }));
    assert!(effects.contains(&PlayerNetworkEffect::CloseConnection { connection_id: 11 }));
}

#[test]
fn ignores_persistence_and_cleanup_effects() {
    let manager = player_manager();

    assert!(
        PlayerEffectNetworkPlan::plan(&PlayerEffect::UpdateLastLogin { user_id: 1 }, &manager,)
            .is_empty()
    );
    assert!(PlayerEffectNetworkPlan::plan(
        &PlayerEffect::DisposeInventory { user_id: 1 },
        &manager,
    )
    .is_empty());
    assert!(PlayerEffectNetworkPlan::plan(
        &PlayerEffect::LeaveCurrentRoom { connection_id: 10 },
        &manager,
    )
    .is_empty());
}
