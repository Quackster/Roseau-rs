use super::room_decoration_incoming_plan::*;
use crate::dao::in_memory::{InMemoryItemDao, InMemoryRoomDao};
use crate::dao::{ItemDao, RoomDao};
use crate::game::item::{Item, ItemDefinition};
use crate::game::room::settings::RoomType;
use crate::game::room::RoomData;

fn definition(id: i32, behaviour: &str) -> ItemDefinition {
    ItemDefinition::new(id, "paper", "red", 1, 1, 1.0, behaviour, "Paper", "", "")
}

fn item(id: i32, definition: ItemDefinition, custom_data: &str) -> Item {
    Item::new(
        id,
        0,
        7,
        "1",
        0,
        0.0,
        0,
        definition,
        "",
        Some(custom_data.to_owned()),
    )
    .unwrap()
}

fn room(id: i32) -> RoomData {
    RoomData::new(
        id,
        false,
        RoomType::Private,
        7,
        "alice",
        "Room",
        0,
        "",
        25,
        "desc",
        "model_a",
        "default",
        "old-wall",
        "old-floor",
        false,
        true,
    )
}

#[test]
fn applies_wallpaper_decoration_and_consumes_item() {
    let item_dao = InMemoryItemDao::new();
    let room_dao = InMemoryRoomDao::new();
    item_dao.insert_item(item(10, definition(5, "V"), "101"));
    room_dao.insert_room(room(42));

    let outcomes = RoomDecorationIncomingPlan::plan(
        &IncomingExecutionEffect::ApplyDecoration {
            decoration: "wallpaper".to_owned(),
            item_id: 10,
        },
        &item_dao,
        &room_dao,
        42,
        true,
    )
    .unwrap();

    assert_eq!(
        outcomes,
        vec![RoomDecorationOutcome::applied("wallpaper", "101")]
    );
    assert_eq!(room_dao.room(42, false).unwrap().unwrap().wall(), "101");
    assert!(item_dao.item(10).unwrap().is_none());
}

#[test]
fn applies_floor_decoration_and_preserves_other_room_fields() {
    let item_dao = InMemoryItemDao::new();
    let room_dao = InMemoryRoomDao::new();
    item_dao.insert_item(item(11, definition(5, "V"), "wood"));
    room_dao.insert_room(room(42));

    let outcomes = RoomDecorationIncomingPlan::plan(
        &IncomingExecutionEffect::ApplyDecoration {
            decoration: "floor".to_owned(),
            item_id: 11,
        },
        &item_dao,
        &room_dao,
        42,
        true,
    )
    .unwrap();
    let room = room_dao.room(42, false).unwrap().unwrap();

    assert_eq!(
        outcomes,
        vec![RoomDecorationOutcome::applied("floor", "wood")]
    );
    assert_eq!(room.wall(), "old-wall");
    assert_eq!(room.floor(), "wood");
}

#[test]
fn ignores_missing_rights_non_decoration_and_unknown_decoration() {
    let item_dao = InMemoryItemDao::new();
    let room_dao = InMemoryRoomDao::new();
    item_dao.insert_item(item(10, definition(5, "V"), "101"));
    item_dao.insert_item(item(11, definition(6, "SIF"), "blue"));
    room_dao.insert_room(room(42));

    let no_rights = RoomDecorationIncomingPlan::plan(
        &IncomingExecutionEffect::ApplyDecoration {
            decoration: "wallpaper".to_owned(),
            item_id: 10,
        },
        &item_dao,
        &room_dao,
        42,
        false,
    )
    .unwrap();
    let non_decoration = RoomDecorationIncomingPlan::plan(
        &IncomingExecutionEffect::ApplyDecoration {
            decoration: "wallpaper".to_owned(),
            item_id: 11,
        },
        &item_dao,
        &room_dao,
        42,
        true,
    )
    .unwrap();
    let unknown = RoomDecorationIncomingPlan::plan(
        &IncomingExecutionEffect::ApplyDecoration {
            decoration: "ceiling".to_owned(),
            item_id: 10,
        },
        &item_dao,
        &room_dao,
        42,
        true,
    )
    .unwrap();

    assert_eq!(no_rights, vec![RoomDecorationOutcome::Ignored]);
    assert_eq!(non_decoration, vec![RoomDecorationOutcome::Ignored]);
    assert_eq!(unknown, vec![RoomDecorationOutcome::Ignored]);
    assert!(item_dao.item(10).unwrap().is_some());
    assert_eq!(
        room_dao.room(42, false).unwrap().unwrap().wall(),
        "old-wall"
    );
}

#[test]
fn ignores_unrelated_decoration_effects() {
    let item_dao = InMemoryItemDao::new();
    let room_dao = InMemoryRoomDao::new();

    assert!(RoomDecorationIncomingPlan::plan(
        &IncomingExecutionEffect::GoAway,
        &item_dao,
        &room_dao,
        42,
        true,
    )
    .unwrap()
    .is_empty());
}
