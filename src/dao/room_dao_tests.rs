use super::room_dao::*;

#[test]
fn create_room_uses_owner_details() {
    let mut details = PlayerDetails::new();
    details.fill_basic(7, "alice", "hello", "hd-100");

    let request = CreateRoom::new(&details, "Room", "Desc", "model_a", 1, true);

    assert_eq!(request.owner_id, 7);
    assert_eq!(request.owner_name, "alice");
    assert_eq!(request.name, "Room");
    assert!(request.show_owner_name);
}
