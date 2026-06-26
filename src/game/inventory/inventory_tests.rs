use super::*;
use crate::game::item::ItemDefinition;

fn item(id: i32) -> Item {
    Item::new(
        id,
        0,
        1,
        "1",
        1,
        0.0,
        0,
        ItemDefinition::new(1, "chair", "red", 1, 1, 1.0, "SFC", "Chair", "", ""),
        "",
        None,
    )
    .unwrap()
}

#[test]
fn paginates_items_like_java_inventory() {
    let inventory = Inventory::with_items((1..=10).map(item));

    assert_eq!(inventory.paginated_items().get(&0).unwrap().len(), 9);
    assert_eq!(inventory.paginated_items().get(&1).unwrap().len(), 1);
}

#[test]
fn creates_second_page_after_java_counter_boundary() {
    let inventory = Inventory::with_items((1..=11).map(item));

    assert_eq!(inventory.paginated_items().get(&0).unwrap().len(), 9);
    assert_eq!(inventory.paginated_items().get(&1).unwrap().len(), 2);
}

#[test]
fn adds_removes_and_refreshes_pages() {
    let mut inventory = Inventory::with_items([item(1), item(2)]);
    assert!(inventory.get_item(2).is_some());

    let removed = inventory.remove_item_by_id(1, true).unwrap();
    assert_eq!(removed.id(), 1);
    inventory.add_item(item(3));

    assert_eq!(inventory.items().len(), 2);
    assert!(inventory.get_item(3).is_some());
}

#[test]
fn refresh_modes_update_cursor_and_wrap_missing_pages() {
    let mut inventory = Inventory::with_items((1..=11).map(item));

    assert!(matches!(
        inventory.refresh("last"),
        InventoryRefresh::Page { cursor: 1, .. }
    ));
    assert!(matches!(
        inventory.refresh("next"),
        InventoryRefresh::Page { cursor: 0, .. }
    ));
    assert!(matches!(
        inventory.refresh("new"),
        InventoryRefresh::Page { cursor: 0, .. }
    ));
}

#[test]
fn refresh_empty_inventory_returns_empty() {
    let mut inventory = Inventory::new();

    assert_eq!(inventory.refresh("new"), InventoryRefresh::Empty);
}
