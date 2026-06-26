use super::*;
use crate::dao::in_memory::InMemoryRoomDao;
use crate::dao::{CreateRoom, RoomChatlog, RoomDao};
use crate::game::player::{Bot, PlayerDetails};
use crate::game::room::model::{Position, RoomModel};
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomConnection, RoomData};

fn private_room(id: i32, owner_id: i32, name: &str) -> RoomData {
    RoomData::new(
        id,
        false,
        RoomType::Private,
        owner_id,
        "alice",
        name,
        0,
        "",
        25,
        "desc",
        "model_a",
        "default",
        "wall",
        "floor",
        false,
        true,
    )
}

fn public_room(id: i32, hidden: bool, name: &str) -> RoomData {
    RoomData::new(
        id,
        hidden,
        RoomType::Public,
        0,
        "",
        name,
        0,
        "",
        100,
        "desc",
        "model_a",
        "default",
        "",
        "",
        false,
        false,
    )
}

#[test]
fn stores_and_queries_rooms_by_type_owner_and_latest() {
    let dao = InMemoryRoomDao::new();
    dao.insert_room(private_room(1, 7, "Old"));
    dao.insert_room(public_room(2, false, "Lobby"));
    dao.insert_room(private_room(3, 7, "New"));

    let mut details = PlayerDetails::new();
    details.fill_basic(7, "alice", "hello", "hd-100");

    assert_eq!(dao.public_rooms(false).unwrap()[0].id(), 2);
    assert_eq!(dao.public_room_ids().unwrap(), vec![2]);
    assert_eq!(dao.player_rooms(&details, false).unwrap().len(), 2);
    assert_eq!(dao.latest_player_rooms(&[1], 0).unwrap()[0].id(), 3);
}

#[test]
fn creates_updates_and_deletes_rooms() {
    let dao = InMemoryRoomDao::new();
    let mut owner = PlayerDetails::new();
    owner.fill_basic(7, "alice", "hello", "hd-100");
    let created = dao
        .create_room(&CreateRoom::new(&owner, "Room", "Desc", "model_a", 1, true))
        .unwrap();

    assert_eq!(created.id(), 1);
    assert_eq!(created.owner_id(), 7);

    let mut changed = created.clone();
    changed.set_name("Changed");
    dao.update_room(&changed).unwrap();
    assert_eq!(
        dao.room(created.id(), false).unwrap().unwrap().name(),
        "Changed"
    );

    dao.delete_room(&changed).unwrap();
    assert!(dao.room(created.id(), false).unwrap().is_none());
}

#[test]
fn stores_rights_connections_models_bots_and_chatlogs() {
    let dao = InMemoryRoomDao::new();
    let model = RoomModel::new("model_a", "00 00", 0, 0, 0, 2, false, false).unwrap();
    dao.insert_model(model);
    dao.insert_connections(1, vec![RoomConnection::new(1, 2, Position::new(3, 4, 0.0))]);
    dao.insert_bots(
        1,
        vec![Bot::new(Position::new(1, 1, 0.0), vec![], vec![], vec![])],
    );
    dao.save_room_rights(1, &[7, 7, 8]).unwrap();
    dao.save_chatlog(&RoomChatlog::new("alice", 1, "CHAT", "hello"))
        .unwrap();

    assert!(dao.model("model_a").unwrap().is_some());
    assert_eq!(dao.room_connections(1).unwrap()[0].to_id(), 2);
    assert_eq!(dao.bots(1).unwrap().len(), 1);
    assert_eq!(dao.room_rights(1).unwrap(), vec![7, 8]);
    assert_eq!(dao.chatlogs()[0].message, "hello");
}
