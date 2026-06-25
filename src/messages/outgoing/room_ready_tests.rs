use super::room_ready::*;

#[test]
fn composes_room_ready_packet() {
    let mut response = RoomReady::new("model_a").compose();

    assert_eq!(response.get(), "#ROOM_READY\rmodel_a##");
}
