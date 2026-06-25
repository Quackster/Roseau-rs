use crate::dao::mysql::{ItemCommandQueries, SqlExecutionKind, SqlParameter};
use crate::game::item::{Item, ItemDefinition};
use crate::messages::IncomingExecutionEffect;

#[test]
fn maps_set_item_data_effect_to_current_item_read() {
    let read_plan = ItemCommandQueries::read_plan(&IncomingExecutionEffect::SetItemData {
        item_id: 42,
        data: "sticky note".to_owned(),
    })
    .unwrap();

    assert_eq!(read_plan.kind(), SqlExecutionKind::ReadRows);
    assert_eq!(read_plan.sql(), "SELECT * FROM items WHERE id = ? LIMIT 1");
    assert_eq!(read_plan.parameters(), &[SqlParameter::Integer(42)]);
}

#[test]
fn maps_item_command_effects_to_current_item_reads() {
    let move_plan = ItemCommandQueries::read_plan(&IncomingExecutionEffect::MoveStuff {
        item_id: 42,
        x: 5,
        y: 6,
        rotation: Some(2),
    })
    .unwrap();
    let set_data_plan = ItemCommandQueries::read_plan(&IncomingExecutionEffect::SetStuffData {
        item_id: 43,
        data_class: "SWITCHON".to_owned(),
        custom_data: "ON".to_owned(),
    })
    .unwrap();
    let return_plan =
        ItemCommandQueries::read_plan(&IncomingExecutionEffect::ReturnItemToInventory {
            item_id: 44,
        })
        .unwrap();

    assert_eq!(move_plan.kind(), SqlExecutionKind::ReadRows);
    assert_eq!(move_plan.sql(), "SELECT * FROM items WHERE id = ? LIMIT 1");
    assert_eq!(move_plan.parameters(), &[SqlParameter::Integer(42)]);
    assert_eq!(set_data_plan.parameters(), &[SqlParameter::Integer(43)]);
    assert_eq!(return_plan.parameters(), &[SqlParameter::Integer(44)]);
    assert_eq!(
        ItemCommandQueries::read_plan(&IncomingExecutionEffect::GoAway),
        None
    );
}

#[test]
fn ignores_item_effects_that_need_runtime_context() {
    assert!(ItemCommandQueries::plan(&IncomingExecutionEffect::GoAway).is_empty());
}

#[test]
fn maps_set_stuff_data_to_java_compatible_extra_data_updates() {
    let item_data = ItemCommandQueries::set_item_data_plan(42, Some("old"), "old plus").unwrap();

    assert_eq!(
        item_data.parameters(),
        &[
            SqlParameter::Text("old plus".to_owned()),
            SqlParameter::Integer(42)
        ]
    );
    assert_eq!(
        ItemCommandQueries::set_item_data_plan(42, Some("old"), "new"),
        None
    );

    let switch_plan = ItemCommandQueries::set_stuff_data_plan(42, "SWITCHON", "BROKEN")
        .expect("switch data should persist");
    let status_plan = ItemCommandQueries::set_stuff_data_plan(43, "STATUS", "BROKEN")
        .expect("status data should persist");

    assert_eq!(
        switch_plan.parameters(),
        &[
            SqlParameter::Text("OFF".to_owned()),
            SqlParameter::Integer(42),
        ]
    );
    assert_eq!(
        status_plan.parameters(),
        &[
            SqlParameter::Text("C".to_owned()),
            SqlParameter::Integer(43)
        ]
    );
    assert_eq!(
        ItemCommandQueries::set_stuff_data_plan(44, "DOOROPEN", "TRUE"),
        None
    );
    assert_eq!(
        ItemCommandQueries::set_stuff_data_plan(45, "DIR", "3"),
        None
    );
}

#[test]
fn maps_post_it_use_to_amount_update_or_delete() {
    let update = ItemCommandQueries::use_post_it_plan(42, 3).unwrap();
    let delete = ItemCommandQueries::use_post_it_plan(42, 1).unwrap();
    let remove = ItemCommandQueries::remove_item_plan(43);

    assert_eq!(
        update.parameters(),
        &[
            SqlParameter::Text("2".to_owned()),
            SqlParameter::Integer(42)
        ]
    );
    assert_eq!(delete.sql(), "DELETE FROM items WHERE id = ?");
    assert_eq!(delete.parameters(), &[SqlParameter::Long(42)]);
    assert_eq!(remove.sql(), "DELETE FROM items WHERE id = ?");
    assert_eq!(remove.parameters(), &[SqlParameter::Long(43)]);
}

#[test]
fn maps_inventory_place_and_return_context_to_item_updates() {
    let wall = ItemCommandQueries::place_wall_item_plan(42, 7, 9, ":w=1,1", "paper");
    let floor = ItemCommandQueries::place_floor_item_plan(43, 7, 9, 2, 3, 1.5, 4, "");
    let moved = ItemCommandQueries::move_floor_item_plan(44, 5, 6, 2.25, 3, "3");
    let return_floor = ItemCommandQueries::return_floor_item_plan(43, 9);
    let return_wall = ItemCommandQueries::return_wall_item_plan(42);

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
        return_wall.sql(),
        "UPDATE items SET room_id = ? WHERE id = ?"
    );
}

#[test]
fn maps_moved_floor_item_to_persisted_position_and_custom_data() {
    let item = Item::new(
        44,
        7,
        9,
        "5",
        6,
        2.25,
        3,
        ItemDefinition::new(5, "chair", "red", 1, 1, 1.0, "SIF", "Chair", "", ""),
        "",
        Some("3".to_owned()),
    )
    .unwrap();
    let wall_item = Item::new(
        45,
        7,
        9,
        ":w=1,1",
        0,
        0.0,
        0,
        ItemDefinition::new(6, "poster", "red", 1, 1, 0.0, "SIW", "Poster", "", ""),
        "",
        Some("paper".to_owned()),
    )
    .unwrap();

    let plan = ItemCommandQueries::moved_item_plan(&item).unwrap();

    assert_eq!(
        plan.sql(),
        "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ? WHERE id = ?"
    );
    assert_eq!(
        plan.parameters(),
        &[
            SqlParameter::Text("3".to_owned()),
            SqlParameter::Text("5".to_owned()),
            SqlParameter::Integer(6),
            SqlParameter::Float(2.25),
            SqlParameter::Integer(3),
            SqlParameter::Integer(44),
        ]
    );
    assert_eq!(ItemCommandQueries::moved_item_plan(&wall_item), None);
}
