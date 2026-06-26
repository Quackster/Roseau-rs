use super::*;
use crate::game::item::ItemDefinition;
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

#[test]
fn refresh_inventory_loads_user_items_into_strip_info() {
    // The runtime passes the session's per-connection inventory list,
    // so the executor no longer filters by owner.
    let items = vec![
        item(1, 7, definition(5, "SF", "chair", "Chair"), "blue"),
        item(2, 7, definition(6, "IJ", "note", "Post-it"), "1"),
    ];

    let execution = InventoryCommandExecutor::refresh_inventory(&items, "new").unwrap();

    let Some(strip_info) = execution.strip_info() else {
        panic!("expected strip info");
    };
    assert_eq!(
        strip_info.compose().get(),
        "#STRIPINFO\rroseau;1;0;S;0;chair;Chair;blue;1;2;red/\rroseau;2;0;I;0;note;Post-it;2;2/##"
    );
}

#[test]
fn refresh_inventory_sends_empty_strip_info_for_missing_items() {
    let execution = InventoryCommandExecutor::refresh_inventory(&[], "new").unwrap();

    let strip_info = execution.strip_info().expect("expected strip info");
    assert_eq!(strip_info.compose().get(), "#STRIPINFO##");
}
