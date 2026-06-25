use crate::dao::mysql::{ItemQueries, SqlParameter};
use crate::game::item::{Item, ItemDefinition};

fn definition(flags: &str) -> ItemDefinition {
    ItemDefinition::new(5, "chair", "red", 1, 1, 1.0, flags, "Chair", "", "")
}

fn item(flags: &str, custom_data: Option<String>) -> Item {
    Item::new(10, 7, 3, "1", 2, 0.5, 4, definition(flags), "", custom_data).unwrap()
}

#[test]
fn builds_definition_public_room_and_room_item_reads() {
    assert_eq!(
        ItemQueries::definitions().sql(),
        "SELECT * FROM item_definitions"
    );

    let public_items = ItemQueries::public_room_items("pool_a");
    assert_eq!(
        public_items.sql(),
        "SELECT * FROM room_public_items WHERE model = ?"
    );
    assert_eq!(
        public_items.parameters(),
        &[SqlParameter::Text("pool_a".to_owned())]
    );

    let room_items = ItemQueries::room_items(8);
    assert_eq!(room_items.sql(), "SELECT * FROM items WHERE room_id = ?");
    assert_eq!(room_items.parameters(), &[SqlParameter::Integer(8)]);
    assert_eq!(ItemQueries::item_table(), "items");
}

#[test]
fn builds_single_item_delete_and_save_update() {
    let lookup = ItemQueries::item(10);
    let delete = ItemQueries::delete_item(10);
    let save = ItemQueries::save_item(&item("", Some("ON".to_owned())));

    assert_eq!(lookup.sql(), "SELECT * FROM items WHERE id = ? LIMIT 1");
    assert_eq!(delete.sql(), "DELETE FROM items WHERE id = ?");
    assert_eq!(
        save.sql(),
        "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ?, room_id = ?, user_id = ? WHERE id = ?"
    );
    assert_eq!(
        save.parameters(),
        &[
            SqlParameter::Text("ON".to_owned()),
            SqlParameter::Text("1".to_owned()),
            SqlParameter::Integer(2),
            SqlParameter::Float(0.5),
            SqlParameter::Integer(4),
            SqlParameter::Integer(7),
            SqlParameter::Integer(3),
            SqlParameter::Integer(10),
        ]
    );
}

#[test]
fn builds_focused_item_mutation_queries() {
    let extra_data = ItemQueries::update_extra_data(42, "ON");
    let wall = ItemQueries::place_wall_item(42, 7, 9, ":w=1,1", "paper");
    let floor = ItemQueries::place_floor_item(43, 7, 9, 2, 3, 1.5, 4, "");
    let moved = ItemQueries::move_floor_item(44, 5, 6, 2.25, 3, "3");
    let return_floor = ItemQueries::return_floor_item_to_inventory(43, 9);
    let return_wall = ItemQueries::return_item_to_inventory(42);

    assert_eq!(
        extra_data.sql(),
        "UPDATE items SET extra_data = ? WHERE id = ?"
    );
    assert_eq!(
        extra_data.parameters(),
        &[
            SqlParameter::Text("ON".to_owned()),
            SqlParameter::Integer(42)
        ]
    );
    assert_eq!(
        wall.sql(),
        "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ?, room_id = ?, user_id = ? WHERE id = ?"
    );
    assert_eq!(
        wall.parameters(),
        &[
            SqlParameter::Text("paper".to_owned()),
            SqlParameter::Text(":w=1,1".to_owned()),
            SqlParameter::Integer(0),
            SqlParameter::Float(0.0),
            SqlParameter::Integer(0),
            SqlParameter::Integer(7),
            SqlParameter::Integer(9),
            SqlParameter::Integer(42),
        ]
    );
    assert_eq!(
        floor.parameters(),
        &[
            SqlParameter::Text(String::new()),
            SqlParameter::Text("2".to_owned()),
            SqlParameter::Integer(3),
            SqlParameter::Float(1.5),
            SqlParameter::Integer(4),
            SqlParameter::Integer(7),
            SqlParameter::Integer(9),
            SqlParameter::Integer(43),
        ]
    );
    assert_eq!(
        moved.sql(),
        "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ? WHERE id = ?"
    );
    assert_eq!(
        moved.parameters(),
        &[
            SqlParameter::Text("3".to_owned()),
            SqlParameter::Text("5".to_owned()),
            SqlParameter::Integer(6),
            SqlParameter::Float(2.25),
            SqlParameter::Integer(3),
            SqlParameter::Integer(44),
        ]
    );
    assert_eq!(
        return_floor.sql(),
        "UPDATE items SET x = ?, y = ?, room_id = ?, user_id = ? WHERE id = ?"
    );
    assert_eq!(
        return_floor.parameters(),
        &[
            SqlParameter::Text("-1".to_owned()),
            SqlParameter::Integer(-1),
            SqlParameter::Integer(0),
            SqlParameter::Integer(9),
            SqlParameter::Integer(43),
        ]
    );
    assert_eq!(
        return_wall.parameters(),
        &[SqlParameter::Integer(0), SqlParameter::Integer(42)]
    );
}

#[test]
fn save_extra_data_matches_java_teleporter_fallback() {
    let numeric = item("X", Some("22".to_owned()));
    let fallback = item("X", Some("room-link".to_owned()));

    assert_eq!(ItemQueries::save_extra_data(&numeric), "22");
    assert_eq!(ItemQueries::save_extra_data(&fallback), "0");
}
