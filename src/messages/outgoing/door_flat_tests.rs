use super::door_flat::*;

#[test]
fn composes_door_flat_packet() {
    let mut response = DoorFlat::new(12, 34).compose();

    assert_eq!(response.get(), "#DOORFLAT\r12\r34##");
}
