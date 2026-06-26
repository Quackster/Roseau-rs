use super::*;

#[test]
fn maps_item_definition_row_to_domain_definition() {
    let row = ItemDefinitionRow::new(
        5, "chair", "red", 1, 1, 1.0, "SWITCHON", "SFC", "Chair", "A chair",
    );

    let definition = item_definition_from_row(&row);

    assert_eq!(definition.id(), 5);
    assert_eq!(definition.sprite(), "chair");
    assert_eq!(definition.data_class(), "SWITCHON");
    assert!(definition.behaviour().can_sit_on_top());
}

#[test]
fn maps_user_row_to_player_details() {
    let row = UserRow::new(
        7,
        "alice",
        "hash",
        4,
        "hello",
        "hd-100",
        "pool",
        "alice@example.test",
        55,
        "F",
        "UK",
        "ADM",
        "1990-01-01",
        1000,
        2000,
        "welcome",
        8,
    );

    let details = player_details_from_row(&row);

    assert_eq!(details.id(), 7);
    assert_eq!(details.username(), "alice");
    assert_eq!(details.password(), "hash");
    assert_eq!(details.pool_figure(), "pool");
    assert_eq!(details.tickets(), 8);
}

#[test]
fn maps_room_model_row_to_domain_model() {
    let row = RoomModelRow::new("model_a", 1, 1, 2, 4, "00 0x", true, false);

    let model = room_model_from_row(&row).unwrap();

    assert_eq!(model.name(), "model_a");
    assert_eq!(model.door_x(), 1);
    assert!(model.has_pool());
}

#[test]
fn maps_messenger_message_and_permission_rows() {
    let message = messenger_message_from_row(&MessengerMessageRow::new(1, 2, 3, 123, "hi", true));
    let permission = permission_from_row(&UserPermissionRow::new(1, 7, "room_admin", true));

    assert_eq!(message.from_id(), 2);
    assert_eq!(message.to_id(), 3);
    assert!(permission.is_inheritable());
    assert_eq!(permission.permission(), "room_admin");
}
