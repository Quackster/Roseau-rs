use super::room_manager::*;
use crate::game::room::settings::RoomType;
use crate::game::room::RoomData;

fn room(id: i32, room_type: RoomType, owner_id: i32, hidden: bool) -> RoomSummary {
    let data = RoomData::new(
        id,
        hidden,
        room_type,
        owner_id,
        "owner",
        format!("room{id}"),
        0,
        "",
        25,
        "desc",
        "model",
        "class",
        "wall",
        "floor",
        false,
        true,
    );
    RoomSummary::new(data)
}

#[test]
fn adds_once_and_finds_loaded_rooms() {
    let mut manager = RoomManager::new();
    let mut public = room(1, RoomType::Public, 7, false);
    public.set_order_id(2);

    assert!(manager.add(public));
    assert!(!manager.add(room(1, RoomType::Public, 7, false)));

    assert_eq!(manager.get_room_by_id(1).unwrap().data().name(), "room1");
    assert_eq!(
        manager.get_room_by_port(37121, 37120).unwrap().data().id(),
        1
    );
    assert_eq!(manager.get_room_by_name("room1").unwrap().data().id(), 1);

    assert_eq!(manager.remove_loaded_room(1).unwrap().data().id(), 1);
    assert!(manager.get_room_by_id(1).is_none());
}

#[test]
fn lists_public_popular_and_owner_rooms_like_java_manager() {
    let mut manager = RoomManager::new();
    let mut public_late = room(1, RoomType::Public, 7, false);
    public_late.set_order_id(2);
    let mut public_first = room(2, RoomType::Public, 8, false);
    public_first.set_order_id(1);
    let mut private_busy = room(3, RoomType::Private, 7, false);
    private_busy.set_player_count(4);
    let mut private_quiet = room(4, RoomType::Private, 7, false);
    private_quiet.set_player_count(1);

    manager.add(public_late);
    manager.add(public_first);
    manager.add(private_quiet);
    manager.add(private_busy);
    manager.add(room(5, RoomType::Public, 7, true));

    let public_ids: Vec<_> = manager
        .get_public_rooms()
        .into_iter()
        .map(|room| room.data().id())
        .collect();
    let popular_ids: Vec<_> = manager
        .get_popular_rooms(0)
        .into_iter()
        .map(|room| room.data().id())
        .collect();
    let owner_ids: Vec<_> = manager
        .get_player_rooms(7)
        .into_iter()
        .map(|room| room.data().id())
        .collect();

    assert_eq!(public_ids, vec![2, 1]);
    assert_eq!(popular_ids, vec![3, 4]);
    assert_eq!(owner_ids, vec![1, 3, 4]);
}
