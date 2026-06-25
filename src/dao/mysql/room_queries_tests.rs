use crate::dao::mysql::{RoomQueries, SqlParameter};
use crate::dao::{CreateRoom, RoomChatlog};
use crate::game::player::PlayerDetails;
use crate::game::room::settings::RoomType;
use crate::game::room::RoomData;

fn owner() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(7, "alice", "hello", "hd-100");
    details
}

fn room_data() -> RoomData {
    RoomData::new(
        42,
        false,
        RoomType::Private,
        7,
        "alice",
        "My room",
        2,
        "secret",
        25,
        "desc",
        "model_a",
        "default",
        "wallpaper",
        "floor",
        true,
        false,
    )
}

#[test]
fn builds_public_room_reads() {
    assert_eq!(
        RoomQueries::public_rooms().sql(),
        "SELECT * FROM rooms WHERE enabled = ? AND room_type = ? ORDER BY order_id ASC"
    );
    assert_eq!(
        RoomQueries::public_room_ids().sql(),
        "SELECT id FROM rooms WHERE enabled = ? AND room_type = ? AND hidden = ? ORDER BY order_id ASC"
    );
    assert_eq!(
        RoomQueries::public_room_ids().parameters(),
        &[
            SqlParameter::Bool(true),
            SqlParameter::Integer(1),
            SqlParameter::Bool(false),
        ]
    );
}

#[test]
fn builds_room_lookup_and_related_reads() {
    assert_eq!(
        RoomQueries::room_connections(5).sql(),
        "SELECT * FROM room_public_connections WHERE room_id = ?"
    );
    assert_eq!(
        RoomQueries::player_rooms(7).sql(),
        "SELECT * FROM rooms WHERE owner_id = ?"
    );
    assert_eq!(
        RoomQueries::latest_player_rooms(2).parameters(),
        &[SqlParameter::Integer(0), SqlParameter::Integer(22)]
    );
    assert_eq!(
        RoomQueries::room(42).sql(),
        "SELECT * FROM rooms WHERE id = ? LIMIT 1"
    );
}

#[test]
fn builds_rights_and_room_mutations() {
    assert_eq!(
        RoomQueries::room_rights(42).sql(),
        "SELECT * FROM room_rights WHERE room_id = ?"
    );
    assert_eq!(
        RoomQueries::delete_room_rights(42).sql(),
        "DELETE FROM room_rights WHERE room_id = ?"
    );
    assert_eq!(
        RoomQueries::insert_room_right(42, 7).sql(),
        "INSERT INTO room_rights (room_id, user_id) VALUES (?, ?)"
    );
    assert_eq!(
        RoomQueries::delete_room(42).sql(),
        "DELETE FROM rooms WHERE id = ?"
    );
}

#[test]
fn builds_create_chatlog_and_update_queries() {
    let create = CreateRoom::new(&owner(), "Room", "Desc", "model_a", 1, true);
    let chatlog = RoomChatlog::new("alice", 42, "SHOUT", "hello");
    let update = RoomQueries::update_room(&room_data());
    let update_flat = RoomQueries::update_flat(42, "Renamed", 1, true);
    let update_flat_info = RoomQueries::update_flat_info(42, "new desc", "open", false);
    let decoration = RoomQueries::update_decoration(42, "wallpaper", "101").unwrap();

    assert_eq!(
        RoomQueries::create_room(&create).sql(),
        "INSERT INTO rooms (name, description, owner_id, model, state, show_owner_name, room_type) VALUES (?, ?, ?, ?, ?, ?, ?)"
    );
    assert_eq!(
        RoomQueries::save_chatlog(&chatlog, 1234).parameters(),
        &[
            SqlParameter::Text("alice".to_owned()),
            SqlParameter::Integer(42),
            SqlParameter::Long(1234),
            SqlParameter::Integer(1),
            SqlParameter::Text("hello".to_owned()),
        ]
    );
    assert_eq!(
        update.sql(),
        "UPDATE rooms SET name = ?, description = ?, state = ?, password = ?, wallpaper = ?, floor = ?, allsuperuser = ?, show_owner_name = ? WHERE id = ?"
    );
    assert_eq!(
        update.parameters(),
        &[
            SqlParameter::Text("My room".to_owned()),
            SqlParameter::Text("desc".to_owned()),
            SqlParameter::Integer(2),
            SqlParameter::Text("secret".to_owned()),
            SqlParameter::Text("wallpaper".to_owned()),
            SqlParameter::Text("floor".to_owned()),
            SqlParameter::Bool(true),
            SqlParameter::Bool(false),
            SqlParameter::Integer(42),
        ]
    );
    assert_eq!(
        update_flat.sql(),
        "UPDATE rooms SET name = ?, state = ?, show_owner_name = ? WHERE id = ?"
    );
    assert_eq!(
        update_flat.parameters(),
        &[
            SqlParameter::Text("Renamed".to_owned()),
            SqlParameter::Integer(1),
            SqlParameter::Bool(true),
            SqlParameter::Integer(42),
        ]
    );
    assert_eq!(
        update_flat_info.sql(),
        "UPDATE rooms SET description = ?, password = ?, allsuperuser = ? WHERE id = ?"
    );
    assert_eq!(
        update_flat_info.parameters(),
        &[
            SqlParameter::Text("new desc".to_owned()),
            SqlParameter::Text("open".to_owned()),
            SqlParameter::Bool(false),
            SqlParameter::Integer(42),
        ]
    );
    assert_eq!(
        decoration.sql(),
        "UPDATE rooms SET wallpaper = ? WHERE id = ?"
    );
    assert_eq!(
        decoration.parameters(),
        &[
            SqlParameter::Text("101".to_owned()),
            SqlParameter::Integer(42)
        ]
    );
    assert_eq!(RoomQueries::update_decoration(42, "ceiling", "x"), None);
}

#[test]
fn builds_bot_model_reads_and_table_names() {
    assert_eq!(
        RoomQueries::bots(42).sql(),
        "SELECT * FROM room_bots WHERE room_id = ?"
    );
    assert_eq!(RoomQueries::models().sql(), "SELECT * FROM room_models");
    assert_eq!(RoomQueries::chat_type_code("CHAT"), 0);
    assert_eq!(RoomQueries::chat_type_code("WHISPER"), 2);
    assert_eq!(
        (
            RoomQueries::room_table(),
            RoomQueries::right_table(),
            RoomQueries::connection_table(),
            RoomQueries::bot_table(),
            RoomQueries::chatlog_table(),
        ),
        (
            "rooms",
            "room_rights",
            "room_public_connections",
            "room_bots",
            "room_chatlogs",
        )
    );
}
