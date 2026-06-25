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
    manager.insert(PlayerSession::new(41, 30001, details(7, "alice")));
    manager.insert(PlayerSession::new(42, 30002, details(8, "bob")));
    manager
}

#[test]
fn plans_room_leave_for_disposed_room_connection() {
    let manager = player_manager();

    let effects = PlayerEffectRoomLeavePlan::plan(
        &PlayerEffect::LeaveCurrentRoom { connection_id: 41 },
        &manager,
    );

    assert_eq!(
        effects,
        vec![RoomEffect::LeaveRoom {
            user_id: 7,
            hotel_view: false,
        }]
    );
}

#[test]
fn ignores_unknown_connections_and_non_room_leave_effects() {
    let manager = player_manager();

    let effects = PlayerEffectRoomLeavePlan::plan_all(
        &[
            PlayerEffect::LeaveCurrentRoom { connection_id: 99 },
            PlayerEffect::SendAlert(SystemBroadcast::new("maintenance")),
            PlayerEffect::DisposeInventory { user_id: 7 },
        ],
        &manager,
    );

    assert!(effects.is_empty());
}
