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

fn wall_place_effect(wall_position: &str) -> IncomingExecutionEffect {
    IncomingExecutionEffect::PlaceWallItemFromInventory {
        item_id: 10,
        wall_position: wall_position.to_owned(),
    }
}

fn floor_place_effect(rotation: i32) -> IncomingExecutionEffect {
    IncomingExecutionEffect::PlaceFloorItemFromInventory {
        item_id: 10,
        x: 5,
        y: 6,
        rotation,
    }
}

#[test]
fn maps_loaded_wall_inventory_item_to_place_plan() {
    let plan = ItemCommandResultMapper::place_item_from_inventory_plan(
        SqlExecutionResult::rows([item_row_with_data(10, 5, "paper")]),
        &definitions("", "SIW"),
        &wall_place_effect(":w=1,1/paper"),
        42,
        99,
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        plan.sql(),
        "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ?, room_id = ?, user_id = ? WHERE id = ?"
    );
    assert_eq!(
        plan.parameters(),
        &[
            SqlParameter::Text("paper".to_owned()),
            SqlParameter::Text(":w=1,1".to_owned()),
            SqlParameter::Integer(0),
            SqlParameter::Float(0.0),
            SqlParameter::Integer(0),
            SqlParameter::Integer(42),
            SqlParameter::Integer(99),
            SqlParameter::Integer(10),
        ]
    );
}

#[test]
fn maps_loaded_floor_inventory_item_to_place_plan() {
    let plan = ItemCommandResultMapper::place_item_from_inventory_plan(
        SqlExecutionResult::rows([item_row_with_data(10, 5, "")]),
        &definitions("", "SIF"),
        &floor_place_effect(3),
        42,
        99,
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        plan.parameters(),
        &[
            SqlParameter::Text(String::new()),
            SqlParameter::Text("5".to_owned()),
            SqlParameter::Integer(6),
            SqlParameter::Float(0.5),
            SqlParameter::Integer(3),
            SqlParameter::Integer(42),
            SqlParameter::Integer(99),
            SqlParameter::Integer(10),
        ]
    );
}

#[test]
fn maps_dir_floor_inventory_place_to_rotation_custom_data() {
    let plan = ItemCommandResultMapper::place_item_from_inventory_plan(
        SqlExecutionResult::rows([item_row_with_data(10, 5, "old")]),
        &definitions("DIR", "SIF"),
        &floor_place_effect(4),
        42,
        99,
    )
    .unwrap()
    .unwrap();

    assert_eq!(
        plan.parameters(),
        &[
            SqlParameter::Text("4".to_owned()),
            SqlParameter::Text("5".to_owned()),
            SqlParameter::Integer(6),
            SqlParameter::Float(0.5),
            SqlParameter::Integer(4),
            SqlParameter::Integer(42),
            SqlParameter::Integer(99),
            SqlParameter::Integer(10),
        ]
    );
}
