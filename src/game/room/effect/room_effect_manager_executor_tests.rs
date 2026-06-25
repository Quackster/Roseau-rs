use super::room_effect_manager_executor::*;
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomData, RoomSummary};

fn room(id: i32) -> RoomSummary {
    RoomSummary::new(RoomData::new(
        id,
        false,
        RoomType::Private,
        7,
        "owner",
        format!("room{id}"),
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

#[test]
fn removes_loaded_room_from_manager() {
    let mut manager = RoomManager::new();
    manager.add(room(11));
    manager.add(room(12));

    let removed = RoomEffectManagerExecutor::apply(
        &mut manager,
        &RoomEffect::RemoveLoadedRoom { room_id: 11 },
    );

    assert!(removed);
    assert!(manager.get_room_by_id(11).is_none());
    assert_eq!(manager.get_room_by_id(12).unwrap().data().name(), "room12");
}

#[test]
fn reports_only_effects_that_mutate_loaded_rooms() {
    let mut manager = RoomManager::new();
    manager.add(room(11));

    let count = RoomEffectManagerExecutor::apply_all(
        &mut manager,
        &[
            RoomEffect::ClearRuntimeData,
            RoomEffect::RemoveLoadedRoom { room_id: 12 },
            RoomEffect::RemoveLoadedRoom { room_id: 11 },
        ],
    );

    assert_eq!(count, 1);
    assert!(manager.loaded_rooms().is_empty());
}
