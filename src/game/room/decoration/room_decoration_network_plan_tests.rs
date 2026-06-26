use super::*;
use crate::game::player::{PlayerDetails, PlayerManager, PlayerSession};

fn session(connection_id: i32, user_id: i32, username: &str) -> PlayerSession {
    let mut details = PlayerDetails::new();
    details.fill_basic(user_id, username, "mission", "figure");
    PlayerSession::new(connection_id, 37120, details)
}

#[test]
fn broadcasts_applied_decoration_to_room_players() {
    let mut manager = PlayerManager::new(Vec::new());
    manager.insert(session(42, 7, "alice"));
    manager.insert(session(43, 8, "bob"));
    manager.insert(session(44, 9, "carol"));
    let effects = RoomDecorationNetworkPlan::plan(
        &RoomDecorationOutcome::applied("floor", "wood"),
        &[7, 8],
        &manager,
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#FLATPROPERTY\rfloor/wood##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 43,
                packet: "#FLATPROPERTY\rfloor/wood##".to_owned(),
            },
        ]
    );
}

#[test]
fn ignored_decoration_has_no_network_effect() {
    let manager = PlayerManager::new(Vec::new());
    assert!(
        RoomDecorationNetworkPlan::plan(&RoomDecorationOutcome::Ignored, &[7], &manager).is_empty()
    );
}
