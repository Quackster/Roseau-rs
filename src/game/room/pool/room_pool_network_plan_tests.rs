use super::room_pool_network_plan::*;
use crate::game::player::{PlayerDetails, PlayerSession};

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

fn player_manager() -> PlayerManager {
    let mut manager = PlayerManager::new(Vec::new());
    manager.insert(PlayerSession::new(70, 30000, details(7, "alice")));
    manager.insert(PlayerSession::new(80, 30000, details(8, "bob")));
    manager
}

#[test]
fn broadcasts_jump_performance_to_room_players() {
    let effects = RoomPoolNetworkPlan::plan(
        &IncomingExecutionEffect::JumpPerformance {
            data: "jump=1".to_owned(),
        },
        "alice",
        &[7, 8],
        &player_manager(),
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#JUMPDATA\ralice\rjump=1##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#JUMPDATA\ralice\rjump=1##".to_owned(),
            },
        ]
    );
}

#[test]
fn broadcasts_splash_position_to_room_players() {
    let effects = RoomPoolNetworkPlan::plan(
        &IncomingExecutionEffect::SplashPosition {
            position: "10,11,0.0".to_owned(),
        },
        "alice",
        &[7],
        &player_manager(),
    );

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 70,
            packet: "#SHOWPROGRAM\rBIGSPLASH POSITION 10,11,0.0##".to_owned(),
        }]
    );
}

#[test]
fn close_pool_change_booth_has_no_direct_network_packet() {
    assert!(RoomPoolNetworkPlan::plan(
        &IncomingExecutionEffect::ClosePoolChangeBooth,
        "alice",
        &[7],
        &player_manager(),
    )
    .is_empty());
}
