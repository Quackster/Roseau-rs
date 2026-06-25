use super::door_out::*;

#[test]
fn composes_door_out_packet() {
    let mut response = DoorOut::new("i:", 7, "alice").compose();

    assert_eq!(response.get(), "#DOOR_OUT\ri:7/alice##");
}
