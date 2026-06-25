use std::collections::HashMap;

use crate::dao::mysql::{ItemCommandResultMapper, SqlExecutionResult, SqlRow, SqlValue};
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
    SqlRow::new([
        ("id", SqlValue::Integer(id)),
        ("user_id", SqlValue::Integer(7)),
        ("item_id", SqlValue::Integer(definition_id)),
        ("room_id", SqlValue::Integer(42)),
        ("x", SqlValue::Text("1".to_owned())),
        ("y", SqlValue::Integer(2)),
        ("z", SqlValue::Float(0.5)),
        ("rotation", SqlValue::Integer(4)),
        ("extra_data", SqlValue::Text("ON".to_owned())),
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

fn return_effect() -> IncomingExecutionEffect {
    IncomingExecutionEffect::ReturnItemToInventory { item_id: 10 }
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
fn ignores_missing_and_non_remove_item_results() {
    assert_eq!(
        ItemCommandResultMapper::remove_item_plan(
            SqlExecutionResult::rows([]),
            &definitions("", "SIF"),
            &IncomingExecutionEffect::RemoveItem { item_id: 10 },
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::remove_item_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("", "SIF"),
            &IncomingExecutionEffect::GoAway,
        )
        .unwrap(),
        None
    );
}

#[test]
fn ignores_missing_mismatched_and_non_place_inventory_results() {
    assert_eq!(
        ItemCommandResultMapper::place_item_from_inventory_plan(
            SqlExecutionResult::rows([]),
            &definitions("", "SIF"),
            &floor_place_effect(3),
            42,
            99,
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::place_item_from_inventory_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("", "SIW"),
            &floor_place_effect(3),
            42,
            99,
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::place_item_from_inventory_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("", "SIF"),
            &wall_place_effect(":w=1,1"),
            42,
            99,
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::place_item_from_inventory_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("", "SIF"),
            &IncomingExecutionEffect::GoAway,
            42,
            99,
        )
        .unwrap(),
        None
    );
}

#[test]
fn ignores_missing_non_placeable_and_non_return_item_results() {
    assert_eq!(
        ItemCommandResultMapper::return_item_to_inventory_plan(
            SqlExecutionResult::rows([]),
            &definitions("", "SIF"),
            &return_effect(),
            99,
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::return_item_to_inventory_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("", ""),
            &return_effect(),
            99,
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::return_item_to_inventory_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("", "SIF"),
            &IncomingExecutionEffect::GoAway,
            99,
        )
        .unwrap(),
        None
    );
}

#[test]
fn ignores_missing_non_post_it_and_non_use_strip_results() {
    assert_eq!(
        ItemCommandResultMapper::use_strip_item_plan(
            SqlExecutionResult::rows([]),
            &definitions("", "SIFJ"),
            &IncomingExecutionEffect::UseStripItem { item_id: 10 },
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::use_strip_item_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("", "SIF"),
            &IncomingExecutionEffect::UseStripItem { item_id: 10 },
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::use_strip_item_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("", "SIFJ"),
            &IncomingExecutionEffect::GoAway,
        )
        .unwrap(),
        None
    );
}

#[test]
fn ignores_missing_non_floor_and_non_move_results() {
    assert_eq!(
        ItemCommandResultMapper::move_stuff_plan(
            SqlExecutionResult::rows([]),
            &definitions("", "SIF"),
            &move_effect(Some(2)),
            None,
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::move_stuff_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("", "SIW"),
            &move_effect(Some(2)),
            None,
        )
        .unwrap(),
        None
    );
    assert_eq!(
        ItemCommandResultMapper::move_stuff_plan(
            SqlExecutionResult::rows([item_row(10, 5)]),
            &definitions("", "SIF"),
            &IncomingExecutionEffect::GoAway,
            None,
        )
        .unwrap(),
        None
    );
}
