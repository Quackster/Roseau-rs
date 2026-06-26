use crate::game::item::{Item, ItemDefinition};
use crate::game::room::model::{Position, RoomModel};
use crate::game::room::{RoomMapping, RoomOccupant};

fn model() -> RoomModel {
    RoomModel::new("model_a", "000 0x0 000", 0, 0, 0, 0, false, false).unwrap()
}

fn item(id: i32, x: &str, y: i32, z: f64, flags: &str, sprite: &str) -> Item {
    Item::new(
        id,
        1,
        1,
        x,
        y,
        z,
        0,
        ItemDefinition::new(id, sprite, "", 1, 1, 1.0, flags, "", "", ""),
        "",
        None,
    )
    .unwrap()
}

#[test]
fn regenerates_tile_heights_and_highest_items() {
    let mut mapping = RoomMapping::new(model());
    let chair = item(1, "1", 0, 0.0, "SFC", "chair");
    let table = item(2, "2", 0, 0.5, "SFH", "table");

    mapping.regenerate_collision_maps([chair.clone(), table.clone()]);

    assert_eq!(mapping.highest_item_id(1, 0), Some(1));
    assert_eq!(mapping.stack_height(1, 0), chair.total_height());
    assert_eq!(mapping.highest_item_id(2, 0), Some(2));
    assert_eq!(mapping.stack_height(2, 0), table.total_height());
    assert_eq!(mapping.tile(1, 0).unwrap().item_ids(), &[1]);
}

#[test]
fn walking_height_ignores_sit_and_lay_surface_height() {
    let mut mapping = RoomMapping::new(model());
    let chair = item(1, "1", 0, 0.25, "SFC", "chair");
    let bed = item(2, "2", 0, 0.5, "SFB", "bed");
    let table = item(3, "0", 0, 0.25, "SFH", "table");
    let items = vec![chair.clone(), bed.clone(), table.clone()];

    mapping.regenerate_collision_maps(items.clone());

    assert_eq!(mapping.stack_height(1, 0), chair.total_height());
    assert_eq!(mapping.walking_height(1, 0, &items), chair.position().z());
    assert_eq!(mapping.stack_height(2, 0), bed.total_height());
    assert_eq!(mapping.walking_height(2, 0, &items), bed.position().z());
    assert_eq!(mapping.walking_height(0, 0, &items), table.total_height());
}

#[test]
fn applies_affected_tile_height_without_lowering_higher_stack() {
    let mut mapping = RoomMapping::new(model());
    let large = Item::new(
        1,
        1,
        1,
        "0",
        0,
        0.0,
        0,
        ItemDefinition::new(1, "sofa", "", 2, 1, 1.0, "SFC", "", "", ""),
        "",
        None,
    )
    .unwrap();

    mapping.regenerate_collision_maps([large.clone()]);

    assert_eq!(mapping.highest_item_id(1, 0), Some(1));
    assert_eq!(mapping.stack_height(1, 0), large.total_height());
}

#[test]
fn validates_tiles_against_model_locks_items_occupants_and_goals() {
    let mut mapping = RoomMapping::new(model());
    let lock = Item::new(
        3,
        1,
        1,
        "0",
        2,
        0.0,
        0,
        ItemDefinition::new(3, "gate", "", 1, 1, 0.0, "SF", "", "", ""),
        "",
        Some("2,2".to_owned()),
    )
    .unwrap();
    let chair = item(1, "1", 0, 0.0, "SFC", "chair");
    let solid = item(2, "0", 1, 0.0, "SF", "solid");
    let items = vec![chair.clone(), solid.clone(), lock.clone()];
    let occupants = vec![
        RoomOccupant::new(10, Position::new(0, 0, 0.0), None),
        RoomOccupant::new(11, Position::new(2, 0, 0.0), Some(Position::new(1, 2, 0.0))),
    ];

    mapping.regenerate_collision_maps(items.clone());

    assert!(mapping.is_valid_tile(10, 1, 0, &items, &occupants, false));
    assert!(!mapping.is_valid_tile(10, 1, 1, &items, &occupants, false));
    assert!(!mapping.is_valid_tile(10, 0, 1, &items, &occupants, false));
    assert!(!mapping.is_valid_tile(12, 0, 0, &items, &occupants, false));
    assert!(!mapping.is_valid_tile(12, 1, 2, &items, &occupants, false));
    assert!(!mapping.is_valid_tile(12, 2, 2, &items, &occupants, false));
}

#[test]
fn finds_nearby_occupants_and_walkway_ids() {
    let mut mapping = RoomMapping::new(model());
    mapping.set_room_walkway_ids(vec![10, 11]);
    let occupants = vec![
        RoomOccupant::new(1, Position::new(0, 0, 0.0), None),
        RoomOccupant::new(2, Position::new(1, 1, 0.0), None),
        RoomOccupant::new(3, Position::new(5, 5, 0.0), None),
    ];

    assert_eq!(mapping.room_walkway_ids(), &[10, 11]);
    assert_eq!(
        mapping.nearby_occupants(1, Position::new(0, 0, 0.0), 2, &occupants),
        vec![occupants[1]]
    );
}

#[test]
fn applies_item_override_locks_to_primary_and_custom_tiles() {
    let mut mapping = RoomMapping::new(model());
    let item = Item::new(
        3,
        1,
        1,
        "0",
        2,
        0.0,
        0,
        ItemDefinition::new(3, "gate", "", 1, 1, 0.0, "SF", "", "", ""),
        "",
        Some("1,1 2,2".to_owned()),
    )
    .unwrap();

    mapping.set_item_tiles_override_lock(&item, true);

    assert!(mapping.tile(0, 2).unwrap().has_override_lock());
    assert!(mapping.tile(1, 1).unwrap().has_override_lock());
    assert!(mapping.tile(2, 2).unwrap().has_override_lock());

    mapping.set_item_tiles_override_lock(&item, false);

    assert!(!mapping.tile(0, 2).unwrap().has_override_lock());
    assert!(!mapping.tile(1, 1).unwrap().has_override_lock());
    assert!(!mapping.tile(2, 2).unwrap().has_override_lock());
}

#[test]
fn applies_java_rotation_only_adjustment_to_same_tile_items_at_or_above_height() {
    let mapping = RoomMapping::new(model());
    let mut moved = item(1, "1", 1, 0.5, "SFC", "chair");
    moved.position_mut().set_rotation(6);
    let same_tile_higher = item(2, "1", 1, 1.0, "SFC", "chair");
    let same_tile_lower = item(3, "1", 1, 0.0, "SFC", "chair");
    let different_tile = item(4, "2", 1, 1.0, "SFC", "chair");
    let mut items = vec![
        moved.clone(),
        same_tile_higher,
        same_tile_lower,
        different_tile,
    ];

    let updated = mapping.apply_rotation_only_item_adjustment(&moved, &mut items);

    assert_eq!(updated.iter().map(Item::id).collect::<Vec<_>>(), vec![2]);
    assert_eq!(items[1].position().rotation(), 6);
    assert_eq!(items[2].position().rotation(), 0);
    assert_eq!(items[3].position().rotation(), 0);
}
