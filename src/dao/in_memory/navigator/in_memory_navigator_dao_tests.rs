use super::*;
use super::*;

fn room(id: i32, room_type: RoomType, name: &str) -> RoomData {
    RoomData::new(
        id, false, room_type, 7, "alice", name, 0, "", 25, "desc", "model", "class", "wall",
        "floor", false, true,
    )
}

#[test]
fn searches_private_rooms_by_case_insensitive_name_part() {
    let dao = InMemoryNavigatorDao::new([
        room(1, RoomType::Private, "Cafe"),
        room(2, RoomType::Private, "Library"),
        room(3, RoomType::Public, "Cafe Public"),
    ]);

    let rooms = dao.rooms_by_like_name("caf").unwrap();

    assert_eq!(rooms.len(), 1);
    assert_eq!(rooms[0].id(), 1);
}
