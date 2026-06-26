use super::*;
use crate::game::room::settings::RoomType;
use crate::game::room::RoomData;
use crate::messages::OutgoingMessage;

fn room(id: i32, owner_name: &str, show_owner_name: bool, player_count: usize) -> RoomSummary {
    let mut room = RoomSummary::new(RoomData::new(
        id,
        false,
        RoomType::Private,
        7,
        owner_name,
        format!("Room {id}"),
        1,
        "",
        25,
        "A room",
        "model",
        "class",
        "wall",
        "floor",
        false,
        show_owner_name,
    ));
    room.set_player_count(player_count);
    room
}

#[test]
fn maps_room_summaries_to_busy_flat_results_packet() {
    let outcome = NavigatorSearchOutcome::new(
        NavigatorRequest::PopularRooms,
        [room(12, "alice", false, 3), room(13, "bob", true, 1)],
        "127.0.0.1",
        37119,
    );
    let mut response = outcome.busy_flat_results().compose();

    assert_eq!(outcome.request(), NavigatorRequest::PopularRooms);
    assert_eq!(outcome.rooms().len(), 2);
    assert_eq!(
        response.get(),
        "#BUSY_FLAT_RESULTS 1\r12/Room 12/-/closed//floor1/127.0.0.1/127.0.0.1/37119/3/null/A room\r13/Room 13/bob/closed//floor1/127.0.0.1/127.0.0.1/37119/1/null/A room##"
    );
}

#[test]
fn keeps_private_room_owner_names_visible() {
    let outcome = NavigatorSearchOutcome::new(
        NavigatorRequest::PrivateRooms,
        [room(12, "alice", false, 0)],
        "10.0.0.1",
        37119,
    );
    let mut response = outcome.busy_flat_results().compose();

    assert_eq!(
        response.get(),
        "#BUSY_FLAT_RESULTS 1\r12/Room 12/alice/closed//floor1/10.0.0.1/10.0.0.1/37119/0/null/A room##"
    );
}

#[test]
fn composes_empty_popular_room_results_like_java_fallback() {
    let outcome = NavigatorSearchOutcome::empty(NavigatorRequest::PopularRooms);
    let mut response = outcome.busy_flat_results().compose();

    assert_eq!(outcome.request(), NavigatorRequest::PopularRooms);
    assert!(outcome.rooms().is_empty());
    assert_eq!(response.get(), "#BUSY_FLAT_RESULTS 1##");
}
