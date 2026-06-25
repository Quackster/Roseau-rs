use super::*;
use crate::game::room::settings::RoomType;
use crate::game::room::RoomData;
use crate::messages::OutgoingMessage;

fn public_room(id: i32, name: &str, class_name: &str, player_count: usize) -> RoomSummary {
    let mut room = RoomSummary::new(RoomData::new(
        id,
        false,
        RoomType::Public,
        -1,
        "",
        name,
        0,
        "",
        25,
        "description",
        "pool_b",
        class_name,
        "wall",
        "floor",
        false,
        true,
    ));
    room.set_player_count(player_count);
    room
}

#[test]
fn maps_public_rooms_to_all_units_for_unit_listener() {
    let outcome = RoomUnitOutcome::listener([public_room(5, "Habbo Lido", "lido", 2)]);
    let mut response = outcome.all_units("127.0.0.1", 22004).unwrap().compose();

    assert_eq!(
        response.get(),
        "#ALLUNITS\rHabbo Lido,2,25,127.0.0.1/127.0.0.1,22009,Habbo Lido\tlido,2,25,pool_b##"
    );
    assert!(outcome.unit_members_packet().is_none());
}

#[test]
fn maps_public_rooms_and_names_to_get_unit_users_packets() {
    let outcome =
        RoomUnitOutcome::unit_members([public_room(5, "Habbo Lido", "lido", 2)], ["alice", "bob"]);
    let mut all_units = outcome.all_units("10.0.0.1", 22004).unwrap().compose();
    let mut unit_members = outcome.unit_members_packet().unwrap().compose();

    assert_eq!(
        all_units.get(),
        "#ALLUNITS\rHabbo Lido,2,25,10.0.0.1/10.0.0.1,22009,Habbo Lido\tlido,2,25,pool_b##"
    );
    assert_eq!(unit_members.get(), "#UNITMEMBERS\ralice\rbob##");
}

#[test]
fn missing_public_room_matches_java_no_packet_path() {
    let outcome = RoomUnitOutcome::missing_room();

    assert!(outcome.all_units("127.0.0.1", 22004).is_none());
    assert!(outcome.unit_members_packet().is_none());
}
