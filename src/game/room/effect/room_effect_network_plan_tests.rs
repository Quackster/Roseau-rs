use super::*;
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
fn maps_room_privilege_and_doorbell_effects_to_packets() {
    let manager = player_manager();

    let effects = RoomEffectNetworkPlan::plan_all(
        &[
            RoomEffect::SendDoorbell {
                user_id: 7,
                username: "visitor".to_owned(),
            },
            RoomEffect::SendOwnerPrivileges { user_id: 7 },
            RoomEffect::SendControllerPrivileges { user_id: 8 },
            RoomEffect::SendNoControllerPrivileges { user_id: 8 },
        ],
        &manager,
    );

    assert_eq!(
        effects,
        vec![
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#DOORBELL_RINGING\rvisitor##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 70,
                packet: "#YOUAREOWNER##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#YOUARECONTROLLER##".to_owned(),
            },
            PlayerNetworkEffect::WriteResponse {
                connection_id: 80,
                packet: "#YOUARENOTCONTROLLER##".to_owned(),
            },
        ]
    );
}

#[test]
fn maps_let_in_and_kick_to_runtime_network_effects() {
    let manager = player_manager();

    assert_eq!(
        RoomEffectNetworkPlan::plan(
            &RoomEffect::LetUserIn {
                user_id: 7,
                room_id: 12,
            },
            &manager,
        ),
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 70,
            packet: "#FLAT_LETIN##".to_owned(),
        }]
    );
    assert_eq!(
        RoomEffectNetworkPlan::plan(&RoomEffect::KickUser { user_id: 8 }, &manager),
        vec![PlayerNetworkEffect::CloseConnection { connection_id: 80 }]
    );
}

#[test]
fn ignores_effects_without_online_target_or_network_side_effect() {
    let manager = player_manager();

    assert!(RoomEffectNetworkPlan::plan(
        &RoomEffect::SendOwnerPrivileges { user_id: 99 },
        &manager,
    )
    .is_empty());
    assert!(RoomEffectNetworkPlan::plan(&RoomEffect::ScheduleWalkTicks, &manager).is_empty());
}
