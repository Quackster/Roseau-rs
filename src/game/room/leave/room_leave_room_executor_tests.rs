use super::room_leave_room_executor::*;
use crate::game::player::{PlayerDetails, PlayerManager, PlayerSession};
use crate::game::room::settings::RoomType;
use crate::game::room::RoomData;

fn private_room(owner_id: i32) -> Room {
    Room::new(RoomData::new(
        12,
        false,
        RoomType::Private,
        owner_id,
        "owner",
        "room",
        0,
        "",
        25,
        "desc",
        "model",
        "class",
        "wall",
        "floor",
        false,
        true,
    ))
}

fn details(user_id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(user_id, username, "mission", "figure");
    details
}

#[test]
fn removes_room_player_for_leave_effect() {
    let manager = PlayerManager::new(Vec::new());
    let mut room = private_room(7);
    room.add_player(7);
    room.add_player(8);

    let effects = RoomLeaveRoomExecutor::apply(
        &mut room,
        &manager,
        &RoomLeaveEffect::RemovePlayerEntity { user_id: 7 },
    );

    assert!(effects.is_empty());
    assert_eq!(room.player_ids(), &[8]);
}

#[test]
fn disposes_empty_private_room_when_owner_is_offline() {
    let manager = PlayerManager::new(Vec::new());
    let mut room = private_room(7);

    let effects = RoomLeaveRoomExecutor::apply(
        &mut room,
        &manager,
        &RoomLeaveEffect::DisposeRoomIfEmpty { room_id: 12 },
    );

    assert_eq!(
        effects,
        vec![
            RoomEffect::ClearRuntimeData,
            RoomEffect::RemoveLoadedRoom { room_id: 12 },
        ]
    );
    assert!(room.is_disposed());
}

#[test]
fn keeps_empty_private_room_loaded_when_owner_is_online() {
    let mut manager = PlayerManager::new(Vec::new());
    manager.insert(PlayerSession::new(99, 30000, details(7, "owner")));
    let mut room = private_room(7);

    let effects = RoomLeaveRoomExecutor::apply(
        &mut room,
        &manager,
        &RoomLeaveEffect::DisposeRoomIfEmpty { room_id: 12 },
    );

    assert_eq!(effects, vec![RoomEffect::ClearRuntimeData]);
    assert!(!room.is_disposed());
}

#[test]
fn applies_room_leave_effects_in_order() {
    let manager = PlayerManager::new(Vec::new());
    let mut room = private_room(7);
    room.add_player(7);

    let effects = RoomLeaveRoomExecutor::apply_all(
        &mut room,
        &manager,
        &[
            RoomLeaveEffect::RemovePlayerEntity { user_id: 7 },
            RoomLeaveEffect::DisposeRoomIfEmpty { room_id: 12 },
        ],
    );

    assert_eq!(
        effects,
        vec![
            RoomEffect::ClearRuntimeData,
            RoomEffect::RemoveLoadedRoom { room_id: 12 },
        ]
    );
    assert!(room.player_ids().is_empty());
    assert!(room.is_disposed());
}
