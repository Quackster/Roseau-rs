use super::room_state::*;

#[test]
fn maps_java_room_state_codes() {
    assert_eq!(RoomState::from_code(0), RoomState::Open);
    assert_eq!(RoomState::from_code(1), RoomState::Doorbell);
    assert_eq!(RoomState::from_code(2), RoomState::Password);
    assert_eq!(RoomState::from_code(99), RoomState::Open);
}

#[test]
fn renders_java_room_state_strings() {
    assert_eq!(RoomState::Open.to_string(), "open");
    assert_eq!(RoomState::Doorbell.to_string(), "closed");
    assert_eq!(RoomState::Password.to_string(), "password");
}
