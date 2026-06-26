use crate::game::item::{Item, ItemDefinition};
use crate::game::room::model::Position;
use crate::protocol::NettyResponse;

fn definition(flags: &str, sprite: &str, data_class: &str) -> ItemDefinition {
    ItemDefinition::new(
        1, sprite, "red", 2, 1, 1.5, flags, "Name", "Desc", data_class,
    )
}

#[test]
fn creates_wall_item_with_wall_position() {
    let item = Item::new(
        10,
        5,
        7,
        "frontwall",
        0,
        0.0,
        0,
        definition("IW", "poster", ""),
        "item-data",
        Some("custom".to_owned()),
    )
    .unwrap();

    assert_eq!(item.position(), Position::new(-1, -1, 0.0));
    assert_eq!(item.wall_position(), Some("frontwall"));
}

#[test]
fn parses_target_teleporter_id_for_teleporters() {
    let item = Item::new(
        10,
        5,
        7,
        "3",
        4,
        0.0,
        2,
        definition("SFX", "teleport", ""),
        "",
        Some("99".to_owned()),
    )
    .unwrap();

    assert_eq!(item.target_teleporter_id(), 99);
}

#[test]
fn truncates_custom_data_and_refreshes_teleporter_id() {
    let mut item = Item::new(
        10,
        5,
        7,
        "3",
        4,
        0.0,
        2,
        definition("SFX", "teleport", ""),
        "",
        Some("1".to_owned()),
    )
    .unwrap();
    item.set_custom_data(format!("123{}", "x".repeat(500)));

    assert_eq!(item.custom_data().unwrap().chars().count(), 400);
    assert_eq!(item.target_teleporter_id(), 0);
}

#[test]
fn serialises_floor_item_like_java() {
    let item = Item::new(
        10,
        5,
        7,
        "3",
        4,
        1.236,
        2,
        definition("SFC", "chair", "state"),
        "",
        Some("open".to_owned()),
    )
    .unwrap();
    let mut response = NettyResponse::with_header("TEST");
    response.append_object(&item);

    assert_eq!(
        response.get(),
        "#TEST\r0000010,chair 3 4 2 1 2 1.24 red/Name/Desc/state/open##"
    );
}

#[test]
fn serialises_wall_item_like_java() {
    let item = Item::new(
        10,
        5,
        7,
        "frontwall",
        0,
        0.0,
        0,
        definition("IW", "poster", ""),
        "",
        Some("note".to_owned()),
    )
    .unwrap();
    let mut response = NettyResponse::with_header("TEST");
    response.append_object(&item);

    assert_eq!(response.get(), "#TEST10;poster;Alex;frontwall\rnote##");
}

#[test]
fn calculates_affected_tiles_and_collision() {
    let item = Item::new(
        10,
        5,
        7,
        "3",
        4,
        1.0,
        2,
        definition("SFC", "chair", ""),
        "",
        None,
    )
    .unwrap();

    assert!(item.has_entity_collision(3, 4));
    assert!(item.has_entity_collision(3, 5));
    assert!(!item.has_entity_collision(6, 6));
    assert_eq!(item.total_height(), 2.5);
}

#[test]
fn checks_java_walkability_rules() {
    let chair = Item::new(
        1,
        0,
        0,
        "1",
        1,
        0.0,
        0,
        definition("SFC", "chair", ""),
        "",
        None,
    )
    .unwrap();
    let pool_lift = Item::new(
        2,
        0,
        0,
        "1",
        1,
        0.0,
        0,
        definition("SF", "poolLift", ""),
        "",
        None,
    )
    .unwrap();
    let teleporter = Item::new(
        3,
        0,
        0,
        "1",
        1,
        0.0,
        0,
        definition("SFX", "teleport", "DOOROPEN"),
        "",
        Some("TRUE".to_owned()),
    )
    .unwrap();

    assert!(chair.can_walk(false));
    assert!(!pool_lift.can_walk(false));
    assert!(pool_lift.can_walk(true));
    assert!(teleporter.can_walk(false));
}

#[test]
fn extracts_lock_coordinate_overrides_from_custom_data() {
    let item = Item::new(
        1,
        0,
        0,
        "1",
        1,
        0.0,
        0,
        definition("SF", "gate", ""),
        "",
        Some("1,2 bad 3,4".to_owned()),
    )
    .unwrap();

    assert_eq!(item.lock_coordinate_overrides(), vec![(1, 2), (3, 4)]);
}
