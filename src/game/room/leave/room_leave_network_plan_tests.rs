use super::room_leave_network_plan::*;
use crate::game::player::{PlayerDetails, PlayerSession};

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

fn player_manager() -> PlayerManager {
    let mut manager = PlayerManager::new(Vec::new());
    manager.insert(PlayerSession::new(70, 30000, details(7, "alice")));
    manager.insert(PlayerSession::new(71, 37119, details(7, "alice")));
    manager.insert(PlayerSession::new(80, 30000, details(8, "bob")));
    manager
}

#[test]
fn closes_private_room_connection_on_matching_private_port() {
    let effects = RoomLeaveNetworkPlan::plan(
        &RoomLeaveEffect::ClosePrivateRoomConnection { user_id: 7 },
        &[7, 8],
        &player_manager(),
        37119,
    );

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::CloseConnection { connection_id: 71 }]
    );
}

#[test]
fn broadcasts_logout_to_current_room_players() {
    let effects = RoomLeaveNetworkPlan::plan(
        &RoomLeaveEffect::BroadcastLogout {
            username: "alice".to_owned(),
        },
        &[8],
        &player_manager(),
        37119,
    );

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 80,
            packet: "#LOGOUT\ralice##".to_owned(),
        }]
    );
}

#[test]
fn plans_room_leave_network_effects_in_order() {
    let effects = RoomLeaveNetworkPlan::plan_all(
        &[
            RoomLeaveEffect::ClosePrivateRoomConnection { user_id: 7 },
            RoomLeaveEffect::RemovePlayerEntity { user_id: 7 },
            RoomLeaveEffect::BroadcastLogout {
                username: "alice".to_owned(),
            },
        ],
        &[8],
        &player_manager(),
        37119,
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::CloseConnection { connection_id: 71 },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#LOGOUT\ralice##".to_owned(),
            },
        ]
    );
}
