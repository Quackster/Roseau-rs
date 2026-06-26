use crate::dao::in_memory::InMemoryItemDao;
use crate::dao::ItemDao;
use crate::game::item::{Item, ItemCommandExecution, ItemCommandExecutor, ItemDefinition};

fn definition(id: i32, flags: &str, data_class: &str) -> ItemDefinition {
    ItemDefinition::new(
        id, "sprite", "red", 1, 1, 1.0, flags, "Name", "", data_class,
    )
}

fn item(id: i32, definition: ItemDefinition, custom_data: &str) -> Item {
    Item::new(
        id,
        0,
        7,
        "1",
        2,
        0.0,
        0,
        definition,
        "",
        Some(custom_data.to_owned()),
    )
    .unwrap()
}

fn room_item(id: i32, room_id: i32, definition: ItemDefinition, custom_data: &str) -> Item {
    let mut item = item(id, definition, custom_data);
    item.set_room_id(room_id);
    item
}

#[test]
fn set_item_data_requires_rights_and_current_data_prefix() {
    let dao = InMemoryItemDao::new();
    dao.insert_item(room_item(1, 42, definition(5, "SIF", ""), "old"));
    dao.insert_item(room_item(2, 99, definition(6, "SIF", ""), "old"));

    assert_eq!(
        ItemCommandExecutor::set_item_data(&dao, 1, 42, "changed", false).unwrap(),
        ItemCommandExecution::Ignored
    );
    assert_eq!(
        ItemCommandExecutor::set_item_data(&dao, 1, 42, "new", true).unwrap(),
        ItemCommandExecution::Ignored
    );
    assert_eq!(
        ItemCommandExecutor::set_item_data(&dao, 2, 42, "old changed", true).unwrap(),
        ItemCommandExecution::Ignored
    );

    let updated = ItemCommandExecutor::set_item_data(&dao, 1, 42, "old plus", true).unwrap();

    let ItemCommandExecution::Updated(item) = updated else {
        panic!("expected update");
    };
    assert_eq!(item.custom_data(), Some("old plus"));
    assert_eq!(
        dao.item(1).unwrap().unwrap().custom_data(),
        Some("old plus")
    );
}

#[test]
fn set_stuff_data_normalises_java_data_classes() {
    let dao = InMemoryItemDao::new();
    dao.insert_item(room_item(1, 42, definition(5, "SIF", "SWITCHON"), "ON"));
    dao.insert_item(room_item(2, 42, definition(6, "SIF", "STATUS"), "O"));
    dao.insert_item(room_item(3, 42, definition(7, "SIF", "DOOROPEN"), "FALSE"));
    dao.insert_item(room_item(4, 42, definition(8, "SIF", "SWITCHON"), "ON"));
    dao.insert_item(room_item(5, 99, definition(9, "SIF", "SWITCHON"), "ON"));

    let switch = ItemCommandExecutor::set_stuff_data(&dao, 1, 42, "SWITCHON", "BROKEN").unwrap();
    let status = ItemCommandExecutor::set_stuff_data(&dao, 2, 42, "STATUS", "BROKEN").unwrap();
    let runtime = ItemCommandExecutor::set_stuff_data(&dao, 3, 42, "DOOROPEN", "BROKEN").unwrap();
    let client_data_class_ignored =
        ItemCommandExecutor::set_stuff_data(&dao, 4, 42, "STATUS", "ON").unwrap();
    let outside_room =
        ItemCommandExecutor::set_stuff_data(&dao, 5, 42, "SWITCHON", "BROKEN").unwrap();

    assert!(matches!(switch, ItemCommandExecution::StuffDataUpdated(_)));
    assert!(matches!(status, ItemCommandExecution::StuffDataUpdated(_)));
    assert_eq!(outside_room, ItemCommandExecution::Ignored);
    assert_eq!(dao.item(1).unwrap().unwrap().custom_data(), Some("OFF"));
    assert_eq!(dao.item(2).unwrap().unwrap().custom_data(), Some("C"));
    let ItemCommandExecution::RuntimeUpdated(runtime) = runtime else {
        panic!("expected runtime update");
    };
    assert_eq!(runtime.custom_data(), Some("FALSE"));
    assert_eq!(dao.item(3).unwrap().unwrap().custom_data(), Some("FALSE"));
    assert!(matches!(
        client_data_class_ignored,
        ItemCommandExecution::StuffDataUpdated(_)
    ));
    assert_eq!(dao.item(4).unwrap().unwrap().custom_data(), Some("ON"));
}

#[test]
fn use_strip_item_decrements_or_deletes_post_it_only() {
    let dao = InMemoryItemDao::new();
    dao.insert_item(item(1, definition(5, "SIFJ", ""), "3"));
    dao.insert_item(item(2, definition(6, "SIFJ", ""), "1"));
    dao.insert_item(item(3, definition(7, "SIF", ""), "3"));

    let updated = ItemCommandExecutor::use_strip_item(&dao, 1).unwrap();
    let deleted = ItemCommandExecutor::use_strip_item(&dao, 2).unwrap();
    let ignored = ItemCommandExecutor::use_strip_item(&dao, 3).unwrap();

    assert!(matches!(updated, ItemCommandExecution::Updated(_)));
    assert_eq!(dao.item(1).unwrap().unwrap().custom_data(), Some("2"));
    assert_eq!(deleted, ItemCommandExecution::Deleted { item_id: 2 });
    assert!(dao.item(2).unwrap().is_none());
    assert_eq!(ignored, ItemCommandExecution::Ignored);
    assert!(dao.item(3).unwrap().is_some());
}

#[test]
fn remove_item_requires_owner_rights() {
    let dao = InMemoryItemDao::new();
    dao.insert_item(room_item(1, 42, definition(5, "SIF", ""), ""));
    dao.insert_item(room_item(2, 99, definition(6, "SIF", ""), ""));

    assert_eq!(
        ItemCommandExecutor::remove_item(&dao, 1, 42, false).unwrap(),
        ItemCommandExecution::Ignored
    );
    assert!(dao.item(1).unwrap().is_some());
    assert_eq!(
        ItemCommandExecutor::remove_item(&dao, 2, 42, true).unwrap(),
        ItemCommandExecution::Ignored
    );
    assert!(dao.item(2).unwrap().is_some());

    let deleted = ItemCommandExecutor::remove_item(&dao, 1, 42, true).unwrap();

    let ItemCommandExecution::RoomItemDeleted(item) = deleted else {
        panic!("expected room item deletion");
    };
    assert_eq!(item.id(), 1);
    assert!(dao.item(1).unwrap().is_none());
}

#[test]
fn returns_floor_and_wall_items_to_inventory_with_owner_rights() {
    let dao = InMemoryItemDao::new();
    dao.insert_item(room_item(1, 42, definition(5, "SIF", ""), ""));
    dao.insert_item(room_item(2, 42, definition(6, "SIW", ""), ""));
    dao.insert_item(room_item(3, 42, definition(7, "", ""), ""));
    dao.insert_item(room_item(4, 99, definition(8, "SIF", ""), ""));

    assert_eq!(
        ItemCommandExecutor::return_item_to_inventory(&dao, 1, 42, 99, false).unwrap(),
        ItemCommandExecution::Ignored
    );
    assert_eq!(
        ItemCommandExecutor::return_item_to_inventory(&dao, 4, 42, 99, true).unwrap(),
        ItemCommandExecution::Ignored
    );
    assert_eq!(dao.item(4).unwrap().unwrap().room_id(), 99);

    let floor = ItemCommandExecutor::return_item_to_inventory(&dao, 1, 42, 99, true).unwrap();
    let wall = ItemCommandExecutor::return_item_to_inventory(&dao, 2, 42, 99, true).unwrap();
    let ignored = ItemCommandExecutor::return_item_to_inventory(&dao, 3, 42, 99, true).unwrap();

    let ItemCommandExecution::RoomItemReturned(floor) = floor else {
        panic!("expected floor return");
    };
    let ItemCommandExecution::RoomItemReturned(wall) = wall else {
        panic!("expected wall return");
    };

    assert_eq!(floor.room_id(), 0);
    assert_eq!(floor.owner_id(), 99);
    assert_eq!(floor.position().x(), -1);
    assert_eq!(floor.position().y(), -1);
    assert_eq!(wall.room_id(), 0);
    assert_eq!(wall.owner_id(), 7);
    assert_eq!(wall.wall_position(), Some("1"));
    assert_eq!(ignored, ItemCommandExecution::Ignored);
    assert_eq!(dao.item(1).unwrap().unwrap().position().x(), -1);
    assert_eq!(dao.item(2).unwrap().unwrap().room_id(), 0);
    assert_eq!(dao.item(3).unwrap().unwrap().room_id(), 42);
}

#[test]
fn places_wall_and_floor_items_with_rights_or_all_super_user() {
    let dao = InMemoryItemDao::new();
    dao.insert_item(item(1, definition(5, "SIW", ""), "paper"));
    dao.insert_item(item(2, definition(6, "SIF", ""), ""));
    dao.insert_item(item(3, definition(7, "SIFX", ""), ""));

    let wall = ItemCommandExecutor::place_wall_item(&dao, 1, 42, 9, ":w=1,1", false, true).unwrap();
    let floor = ItemCommandExecutor::place_floor_item(&dao, 2, 42, 9, 3, 4, true, false).unwrap();
    let teleporter =
        ItemCommandExecutor::place_floor_item(&dao, 3, 42, 9, 5, 6, true, false).unwrap();

    let ItemCommandExecution::RoomItemPlaced(wall) = wall else {
        panic!("expected wall update");
    };
    let ItemCommandExecution::RoomItemPlaced(floor) = floor else {
        panic!("expected floor update");
    };
    let ItemCommandExecution::RoomItemPlaced(teleporter) = teleporter else {
        panic!("expected teleporter update");
    };
    assert_eq!(wall.room_id(), 42);
    assert_eq!(wall.owner_id(), 9);
    assert_eq!(wall.wall_position(), Some(":w=1,1"));
    assert_eq!(floor.position().x(), 3);
    assert_eq!(floor.position().y(), 4);
    assert_eq!(floor.position().rotation(), 0);
    assert_eq!(teleporter.position().rotation(), 4);
}

#[test]
fn place_items_apply_dir_custom_data_from_rotation_like_java_mapping_add() {
    let dao = InMemoryItemDao::new();
    dao.insert_item(item(1, definition(5, "SIW", "DIR"), "old"));
    dao.insert_item(item(2, definition(6, "SIFX", "DIR"), "old"));

    let wall =
        ItemCommandExecutor::place_wall_item(&dao, 1, 42, 9, ":w=1,1/old", true, false).unwrap();
    let floor = ItemCommandExecutor::place_floor_item(&dao, 2, 42, 9, 3, 4, true, false).unwrap();

    let ItemCommandExecution::RoomItemPlaced(wall) = wall else {
        panic!("expected wall update");
    };
    let ItemCommandExecution::RoomItemPlaced(floor) = floor else {
        panic!("expected floor update");
    };
    assert_eq!(wall.custom_data(), Some("0"));
    assert_eq!(floor.custom_data(), Some("4"));
    assert_eq!(dao.item(1).unwrap().unwrap().custom_data(), Some("0"));
    assert_eq!(dao.item(2).unwrap().unwrap().custom_data(), Some("4"));
}

#[test]
fn strips_wall_item_custom_data_suffix_like_java_placement() {
    let dao = InMemoryItemDao::new();
    dao.insert_item(item(1, definition(5, "SIW", ""), "paper"));

    let placed =
        ItemCommandExecutor::place_wall_item(&dao, 1, 42, 9, ":w=1,1/paper", true, false).unwrap();

    let ItemCommandExecution::RoomItemPlaced(item) = placed else {
        panic!("expected wall update");
    };
    assert_eq!(item.wall_position(), Some(":w=1,1"));
    assert_eq!(
        dao.item(1).unwrap().unwrap().wall_position(),
        Some(":w=1,1")
    );
}

#[test]
fn move_stuff_updates_position_and_optional_rotation() {
    let dao = InMemoryItemDao::new();
    dao.insert_item(room_item(1, 42, definition(5, "SIF", ""), ""));
    dao.insert_item(room_item(2, 99, definition(6, "SIF", ""), ""));

    let outside_room =
        ItemCommandExecutor::move_stuff(&dao, 2, 4, 5, Some(2), None, 42, true, false).unwrap();
    let moved =
        ItemCommandExecutor::move_stuff(&dao, 1, 4, 5, Some(2), None, 42, false, true).unwrap();

    assert_eq!(outside_room, ItemCommandExecution::Ignored);
    assert_eq!(dao.item(2).unwrap().unwrap().position().x(), 1);
    let ItemCommandExecution::RoomItemMoved(item) = moved else {
        panic!("expected movement update");
    };
    assert_eq!(item.position().x(), 4);
    assert_eq!(item.position().y(), 5);
    assert_eq!(item.position().rotation(), 2);
    assert_eq!(item.custom_data(), Some(""));
    assert_eq!(dao.item(1).unwrap().unwrap().position().x(), 4);
    assert_eq!(dao.item(1).unwrap().unwrap().custom_data(), Some(""));
}

#[test]
fn move_stuff_applies_dir_random_rotation_like_java_mapping_update() {
    let dao = InMemoryItemDao::new();
    dao.insert_item(room_item(1, 42, definition(5, "SIF", "DIR"), "old"));

    let moved =
        ItemCommandExecutor::move_stuff(&dao, 1, 4, 5, Some(2), Some(6), 42, true, false).unwrap();

    let ItemCommandExecution::RoomItemMoved(item) = moved else {
        panic!("expected movement update");
    };
    assert_eq!(item.position().rotation(), 6);
    assert_eq!(item.custom_data(), Some("6"));
    let persisted = dao.item(1).unwrap().unwrap();
    assert_eq!(persisted.position().rotation(), 6);
    assert_eq!(persisted.custom_data(), Some("6"));
}
