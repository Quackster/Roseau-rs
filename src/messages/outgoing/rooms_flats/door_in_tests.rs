use super::*;

#[test]
fn composes_door_in_packet() {
    let mut response = DoorIn::new("i:", 7, "alice").compose();

    assert_eq!(response.get(), "#DOOR_IN\ri:7/alice##");
}
