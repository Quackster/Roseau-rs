use super::*;
use crate::game::item::{Item, ItemDefinition};
use crate::messages::OutgoingMessage;

fn definition(id: i32, flags: &str, sprite: &str, name: &str) -> ItemDefinition {
    ItemDefinition::new(id, sprite, "red", 1, 2, 1.0, flags, name, "", "")
}

fn item(id: i32, owner_id: i32, definition: ItemDefinition, custom_data: &str) -> Item {
    Item::new(
        id,
        0,
        owner_id,
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

fn inventory_items() -> Vec<Item> {
    vec![item(1, 7, definition(5, "SF", "chair", "Chair"), "blue")]
}

#[test]
fn plans_inventory_refresh_execution_from_incoming_effect() {
    let executions = InventoryIncomingPlan::plan(
        &IncomingExecutionEffect::RefreshInventory {
            category: "new".to_owned(),
        },
        &inventory_items(),
    )
    .unwrap();

    let Some(strip_info) = executions[0].strip_info() else {
        panic!("expected strip info");
    };
    assert_eq!(
        strip_info.compose().get(),
        "#STRIPINFO\rroseau;1;0;S;0;chair;Chair;blue;1;2;red/##"
    );
}

#[test]
fn ignores_unrelated_incoming_effects() {
    assert!(
        InventoryIncomingPlan::plan(&IncomingExecutionEffect::GoAway, &inventory_items())
            .unwrap()
            .is_empty()
    );
}

#[test]
fn plans_all_inventory_refresh_effects_in_order() {
    let executions = InventoryIncomingPlan::plan_all(
        &[
            IncomingExecutionEffect::GoAway,
            IncomingExecutionEffect::RefreshInventory {
                category: "new".to_owned(),
            },
            IncomingExecutionEffect::RefreshInventory {
                category: "last".to_owned(),
            },
        ],
        &inventory_items(),
    )
    .unwrap();

    assert_eq!(executions.len(), 2);
    assert!(executions
        .iter()
        .all(|execution| execution.strip_info().is_some()));
}
