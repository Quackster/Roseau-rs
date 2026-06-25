use super::room_data::*;

fn room_data() -> RoomData {
    RoomData::new(
        42,
        false,
        RoomType::Private,
        7,
        "alice",
        "My room",
        2,
        "secret",
        25,
        "desc",
        "model_a",
        "default",
        "wallpaper",
        "floor",
        false,
        true,
    )
}

#[test]
fn stores_room_data_fields() {
    let data = room_data();

    assert_eq!(data.id(), 42);
    assert_eq!(data.room_type(), RoomType::Private);
    assert_eq!(data.owner_id(), 7);
    assert_eq!(data.owner_name(), "alice");
    assert_eq!(data.name(), "My room");
    assert_eq!(data.state(), RoomState::Password);
    assert_eq!(data.password(), "secret");
    assert_eq!(data.users_max(), 25);
    assert_eq!(data.model_name(), "model_a");
    assert_eq!(data.class_name(), "default");
    assert_eq!(data.wall_height(), -1);
    assert_eq!(data.server_port(30001), 30043);
    assert!(data.show_owner_name());
}

#[test]
fn mutates_room_data_fields() {
    let mut data = room_data();
    data.set_name("Other");
    data.set_state(1);
    data.set_password("door");
    data.set_description("changed");
    data.set_wall("new-wall");
    data.set_floor("new-floor");
    data.set_all_super_user(true);
    data.set_show_owner_name(false);

    assert_eq!(data.name(), "Other");
    assert_eq!(data.state(), RoomState::Doorbell);
    assert_eq!(data.password(), "door");
    assert_eq!(data.description(), "changed");
    assert_eq!(data.wall(), "new-wall");
    assert_eq!(data.floor(), "new-floor");
    assert!(data.has_all_super_user());
    assert!(!data.show_owner_name());
}
