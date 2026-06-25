use super::in_memory_item_dao::*;

pub fn chair_definition() -> ItemDefinition {
    ItemDefinition::new(5, "chair", "red", 1, 1, 1.0, "SFC", "Chair", "", "SWITCHON")
}

pub fn chair_item(id: i32, room_id: i32, owner_id: i32) -> Item {
    Item::new(
        id,
        room_id,
        owner_id,
        "1",
        2,
        0.0,
        4,
        chair_definition(),
        "",
        Some("ON".to_owned()),
    )
    .unwrap()
}

#[test]
fn stores_definitions_and_room_items() {
    let dao = InMemoryItemDao::new();
    dao.insert_definition(chair_definition());
    dao.insert_item(chair_item(10, 7, 3));
    dao.insert_item(chair_item(11, 0, 3));

    assert_eq!(dao.definitions().unwrap().len(), 1);
    assert_eq!(dao.room_items(7).unwrap().len(), 1);
    assert_eq!(dao.item(10).unwrap().unwrap().owner_id(), 3);
}

#[test]
fn saves_and_deletes_items() {
    let dao = InMemoryItemDao::new();
    let mut item = chair_item(10, 7, 3);
    dao.save_item(&item).unwrap();

    item.set_room_id(0);
    dao.save_item(&item).unwrap();

    assert_eq!(dao.item(10).unwrap().unwrap().room_id(), 0);
    dao.delete_item(10).unwrap();
    assert!(dao.item(10).unwrap().is_none());
}
