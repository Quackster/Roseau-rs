use std::collections::HashMap;

use crate::dao::mysql::{
    ItemCommandResultMapper, SqlExecutionResult, SqlParameter, SqlRow, SqlValue,
};
use crate::game::item::ItemDefinition;
use crate::messages::IncomingExecutionEffect;

fn definitions(data_class: &str, behaviour: &str) -> HashMap<i32, ItemDefinition> {
    [(
        5,
        ItemDefinition::new(
            5, "chair", "red", 1, 1, 1.0, behaviour, "Chair", "", data_class,
        ),
    )]
    .into_iter()
    .collect()
}

fn item_row(id: i32, definition_id: i32) -> SqlRow {
    item_row_with_data(id, definition_id, "ON")
}

fn item_row_with_data(id: i32, definition_id: i32, custom_data: &str) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(id)),
        ("user_id", SqlValue::Integer(7)),
        ("item_id", SqlValue::Integer(definition_id)),
        ("room_id", SqlValue::Integer(42)),
        ("x", SqlValue::Text("1".to_owned())),
        ("y", SqlValue::Integer(2)),
        ("z", SqlValue::Float(0.5)),
        ("rotation", SqlValue::Integer(4)),
        ("extra_data", SqlValue::Text(custom_data.to_owned())),
    ])
}

fn move_effect(rotation: Option<i32>) -> IncomingExecutionEffect {
    IncomingExecutionEffect::MoveStuff {
        item_id: 10,
        x: 5,
        y: 6,
        rotation,
    }
}

fn set_stuff_effect(data_class: &str, custom_data: &str) -> IncomingExecutionEffect {
    IncomingExecutionEffect::SetStuffData {
        item_id: 10,
        data_class: data_class.to_owned(),
        custom_data: custom_data.to_owned(),
    }
}

fn set_item_effect(data: &str) -> IncomingExecutionEffect {
    IncomingExecutionEffect::SetItemData {
        item_id: 10,
        data: data.to_owned(),
    }
}

fn return_effect() -> IncomingExecutionEffect {
    IncomingExecutionEffect::ReturnItemToInventory { item_id: 10 }
}

#[test]
fn maps_loaded_floor_item_and_move_effect_to_mutation_plan() {
    let plan = ItemCommandResultMapper::move_stuff_plan(
        SqlExecutionResult::rows([item_row(10, 5)]),
        &definitions("", "SIF"),
        &move_effect(Some(2)),
        None,
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        plan.sql(),
        "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ? WHERE id = ?"
    );
    assert_eq!(
        plan.parameters(),
        &[
            SqlParameter::Text(String::new()),
            SqlParameter::Text("5".to_owned()),
            SqlParameter::Integer(6),
            SqlParameter::Float(0.5),
            SqlParameter::Integer(2),
            SqlParameter::Integer(10),
        ]
    );
}

#[test]
fn maps_dir_move_effect_to_sampled_rotation_and_custom_data() {
    let plan = ItemCommandResultMapper::move_stuff_plan(
        SqlExecutionResult::rows([item_row(10, 5)]),
        &definitions("DIR", "SIF"),
        &move_effect(Some(2)),
        Some(6),
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        plan.parameters(),
        &[
            SqlParameter::Text("6".to_owned()),
            SqlParameter::Text("5".to_owned()),
            SqlParameter::Integer(6),
            SqlParameter::Float(0.5),
            SqlParameter::Integer(6),
            SqlParameter::Integer(10),
        ]
    );
}

#[test]
fn maps_loaded_post_it_use_to_amount_update_or_delete() {
    let update = ItemCommandResultMapper::use_strip_item_plan(
        SqlExecutionResult::rows([item_row_with_data(10, 5, "3")]),
        &definitions("", "SIFJ"),
        &IncomingExecutionEffect::UseStripItem { item_id: 10 },
    )
    .unwrap()
    .unwrap();
    let delete = ItemCommandResultMapper::use_strip_item_plan(
        SqlExecutionResult::rows([item_row_with_data(11, 5, "1")]),
        &definitions("", "SIFJ"),
        &IncomingExecutionEffect::UseStripItem { item_id: 11 },
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        update.parameters(),
        &[
            SqlParameter::Text("2".to_owned()),
            SqlParameter::Integer(10)
        ]
    );
    assert_eq!(delete.sql(), "DELETE FROM items WHERE id = ?");
    assert_eq!(delete.parameters(), &[SqlParameter::Long(11)]);
}

#[test]
fn maps_loaded_item_set_item_data_with_java_prefix_guard() {
    let plan = ItemCommandResultMapper::set_item_data_plan(
        SqlExecutionResult::rows([item_row_with_data(10, 5, "old")]),
        &definitions("", "SIF"),
        &set_item_effect("old plus"),
    )
    .unwrap()
    .unwrap();

    assert_eq!(plan.sql(), "UPDATE items SET extra_data = ? WHERE id = ?");
    assert_eq!(
        plan.parameters(),
        &[
            SqlParameter::Text("old plus".to_owned()),
            SqlParameter::Integer(10)
        ]
    );
    assert_eq!(
        ItemCommandResultMapper::set_item_data_plan(
            SqlExecutionResult::rows([item_row_with_data(10, 5, "old")]),
            &definitions("", "SIF"),
            &set_item_effect("new"),
        )
        .unwrap(),
        None
    );
}

#[test]
fn maps_loaded_item_set_stuff_data_using_definition_data_class() {
    let switch = ItemCommandResultMapper::set_stuff_data_plan(
        SqlExecutionResult::rows([item_row(10, 5)]),
        &definitions("SWITCHON", "SIF"),
        &set_stuff_effect("STATUS", "BROKEN"),
    )
    .unwrap()
    .unwrap();
    let status = ItemCommandResultMapper::set_stuff_data_plan(
        SqlExecutionResult::rows([item_row(11, 5)]),
        &definitions("STATUS", "SIF"),
        &set_stuff_effect("SWITCHON", "BROKEN"),
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        switch.parameters(),
        &[
            SqlParameter::Text("OFF".to_owned()),
            SqlParameter::Integer(10),
        ]
    );
    assert_eq!(
        status.parameters(),
        &[
            SqlParameter::Text("C".to_owned()),
            SqlParameter::Integer(11)
        ]
    );
}

#[test]
fn skips_transient_or_ignored_set_stuff_data_persistence() {
    assert_eq!(
        ItemCommandResultMapper::set_item_data_plan(
            SqlExecutionResult::rows([]),
            &definitions("", "SIF"),
            &set_item_effect("old"),
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::set_item_data_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("", "SIF"),
            &IncomingExecutionEffect::GoAway,
        )
        .unwrap(),
        None
    );

    assert_eq!(
        ItemCommandResultMapper::set_stuff_data_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("DOOROPEN", "SIF"),
            &set_stuff_effect("DOOROPEN", "TRUE"),
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::set_stuff_data_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("DIR", "SIF"),
            &set_stuff_effect("DIR", "3"),
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::set_stuff_data_plan(
            SqlExecutionResult::rows([]),
            &definitions("SWITCHON", "SIF"),
            &set_stuff_effect("SWITCHON", "ON"),
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::set_stuff_data_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("SWITCHON", "SIF"),
            &IncomingExecutionEffect::GoAway,
        )
        .unwrap(),
        None
    );
}

#[test]
fn maps_loaded_remove_item_to_delete_plan() {
    let plan = ItemCommandResultMapper::remove_item_plan(
        SqlExecutionResult::rows([item_row(10, 5)]),
        &definitions("", "SIF"),
        &IncomingExecutionEffect::RemoveItem { item_id: 10 },
    )
    .unwrap()
    .unwrap();

    assert_eq!(plan.sql(), "DELETE FROM items WHERE id = ?");
    assert_eq!(plan.parameters(), &[SqlParameter::Long(10)]);
}

#[test]
fn maps_loaded_floor_item_return_to_inventory_plan() {
    let plan = ItemCommandResultMapper::return_item_to_inventory_plan(
        SqlExecutionResult::rows([item_row(10, 5)]),
        &definitions("", "SIF"),
        &return_effect(),
        99,
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        plan.sql(),
        "UPDATE items SET x = ?, y = ?, room_id = ?, user_id = ? WHERE id = ?"
    );
    assert_eq!(
        plan.parameters(),
        &[
            SqlParameter::Text("-1".to_owned()),
            SqlParameter::Integer(-1),
            SqlParameter::Integer(0),
            SqlParameter::Integer(99),
            SqlParameter::Integer(10),
        ]
    );
}

#[test]
fn maps_loaded_wall_item_return_to_inventory_plan() {
    let plan = ItemCommandResultMapper::return_item_to_inventory_plan(
        SqlExecutionResult::rows([item_row(10, 5)]),
        &definitions("", "SIW"),
        &return_effect(),
        99,
    )
    .unwrap()
    .unwrap();

    assert_eq!(plan.sql(), "UPDATE items SET room_id = ? WHERE id = ?");
    assert_eq!(
        plan.parameters(),
        &[SqlParameter::Integer(0), SqlParameter::Integer(10)]
    );
}
