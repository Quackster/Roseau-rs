use super::*;

#[test]
fn stores_room_connection_fields() {
    let mut connection = RoomConnection::new(1, 2, Position::new(3, 4, 0.0));

    assert_eq!(connection.room_id(), 1);
    assert_eq!(connection.to_id(), 2);
    assert_eq!(connection.door_position(), Position::new(3, 4, 0.0));

    connection.set_room_id(5);
    connection.set_to_id(6);

    assert_eq!(connection.room_id(), 5);
    assert_eq!(connection.to_id(), 6);
}
