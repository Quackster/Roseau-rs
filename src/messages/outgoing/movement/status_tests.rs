use super::*;

#[test]
fn composes_status_packet() {
    let mut response = Status::new([StatusEntity::new(
        "alice",
        1,
        2,
        "3.0",
        4,
        5,
        [RoomUserStatus::new("sit", " 1.0")],
    )])
    .compose();

    assert_eq!(response.get(), "#STATUS \ralice 1,2,3.0,4,5/sit 1.0/##");
}
