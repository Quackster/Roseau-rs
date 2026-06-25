use super::room_command_outcome::*;
use crate::game::room::settings::RoomType;
use crate::messages::OutgoingMessage;

fn room_data() -> RoomData {
    RoomData::new(
        42,
        false,
        RoomType::Private,
        7,
        "alice",
        "Tea Room",
        0,
        "",
        25,
        "description",
        "model_a",
        "class",
        "wall",
        "floor",
        false,
        true,
    )
}

#[test]
fn maps_created_room_to_flat_created_packet() {
    let outcome = RoomCommandOutcome::created(&room_data());
    let mut response = outcome.flat_created().unwrap().compose();

    assert_eq!(response.get(), "#FLATCREATED\r42 Tea Room##");
    assert!(outcome.flat_info_packet().is_none());
}

#[test]
fn maps_loaded_room_to_flat_info_packet() {
    let outcome = RoomCommandOutcome::flat_info(&room_data());
    let mut response = outcome.flat_info_packet().unwrap().compose();

    assert_eq!(response.get(), "#SETFLATINFO\r/42/##");
    assert!(outcome.flat_created().is_none());
}

#[test]
fn ignored_room_command_has_no_packet() {
    assert!(RoomCommandOutcome::Ignored.flat_created().is_none());
    assert!(RoomCommandOutcome::Ignored.flat_info_packet().is_none());
}
