use super::navigator_command_executor::*;
use crate::dao::in_memory::{InMemoryNavigatorDao, InMemoryRoomDao};
use crate::game::player::{PlayerDetails, PlayerSession};
use crate::game::room::settings::RoomType;
use crate::game::room::RoomData;

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
        "model",
        "class",
        "wall",
        "floor",
        false,
        true,
    )
}

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

fn outcome_packets(outcome: &NavigatorSearchOutcome) -> String {
    use crate::messages::OutgoingMessage;

    let mut packet = outcome.busy_flat_results().compose();
    packet.get()
}

#[test]
fn executes_flat_name_search_through_navigator_dao() {
    let navigator_dao = InMemoryNavigatorDao::new(vec![private_room(10, 7, "Cafe")]);
    let room_dao = InMemoryRoomDao::new();
    let room_manager = RoomManager::new();
    let player_manager = PlayerManager::new(vec![]);

    let outcome = NavigatorCommandExecutor::execute(
        &IncomingExecutionEffect::SearchFlat {
            query: "caf".to_owned(),
        },
        &navigator_dao,
        &room_dao,
        &room_manager,
        &player_manager,
        "127.0.0.1",
        37119,
    )
    .unwrap()
    .unwrap();

    assert_eq!(outcome.request(), NavigatorRequest::SearchRooms);
    assert_eq!(outcome.rooms().len(), 1);
    assert!(outcome_packets(&outcome).contains("Cafe"));
}

#[test]
fn executes_busy_flat_search_with_loaded_and_latest_rooms() {
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let room_dao = InMemoryRoomDao::new();
    room_dao.insert_room(private_room(20, 8, "Latest"));
    let mut room_manager = RoomManager::new();
    let mut loaded = RoomSummary::new(private_room(10, 7, "Loaded"));
    loaded.set_player_count(3);
    room_manager.add(loaded);
    let player_manager = PlayerManager::new(vec![]);

    let outcome = NavigatorCommandExecutor::execute(
        &IncomingExecutionEffect::SearchBusyFlats { multiplier: 0 },
        &navigator_dao,
        &room_dao,
        &room_manager,
        &player_manager,
        "127.0.0.1",
        37119,
    )
    .unwrap()
    .unwrap();

    assert_eq!(outcome.request(), NavigatorRequest::PopularRooms);
    assert_eq!(
        outcome
            .rooms()
            .iter()
            .map(|room| room.data().id())
            .collect::<Vec<_>>(),
        vec![10, 20]
    );
}

#[test]
fn executes_empty_busy_flat_search_as_java_fallback_packet() {
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let room_dao = InMemoryRoomDao::new();
    let room_manager = RoomManager::new();
    let player_manager = PlayerManager::new(vec![]);

    let outcome = NavigatorCommandExecutor::execute(
        &IncomingExecutionEffect::EmptySearchBusyFlats,
        &navigator_dao,
        &room_dao,
        &room_manager,
        &player_manager,
        "127.0.0.1",
        37119,
    )
    .unwrap()
    .unwrap();

    assert_eq!(outcome.request(), NavigatorRequest::PopularRooms);
    assert!(outcome.rooms().is_empty());
    assert_eq!(outcome_packets(&outcome), "#BUSY_FLAT_RESULTS 1##");
}

#[test]
fn executes_online_user_room_search_through_room_dao() {
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let room_dao = InMemoryRoomDao::new();
    room_dao.insert_room(private_room(30, 9, "Owned"));
    let room_manager = RoomManager::new();
    let mut player_manager = PlayerManager::new(vec![]);
    player_manager.insert(PlayerSession::new(70, 30000, details(9, "Alice")));

    let outcome = NavigatorCommandExecutor::execute(
        &IncomingExecutionEffect::SearchFlatForUser {
            username: "alice".to_owned(),
        },
        &navigator_dao,
        &room_dao,
        &room_manager,
        &player_manager,
        "127.0.0.1",
        37119,
    )
    .unwrap()
    .unwrap();

    assert_eq!(outcome.request(), NavigatorRequest::PrivateRooms);
    assert_eq!(outcome.rooms()[0].data().id(), 30);
}

#[test]
fn ignores_missing_user_and_non_navigator_effects() {
    let navigator_dao = InMemoryNavigatorDao::new([]);
    let room_dao = InMemoryRoomDao::new();
    let room_manager = RoomManager::new();
    let player_manager = PlayerManager::new(vec![]);

    assert!(NavigatorCommandExecutor::execute(
        &IncomingExecutionEffect::SearchFlatForUser {
            username: "alice".to_owned(),
        },
        &navigator_dao,
        &room_dao,
        &room_manager,
        &player_manager,
        "127.0.0.1",
        37119,
    )
    .unwrap()
    .is_none());
    assert!(NavigatorCommandExecutor::execute(
        &IncomingExecutionEffect::GoAway,
        &navigator_dao,
        &room_dao,
        &room_manager,
        &player_manager,
        "127.0.0.1",
        37119,
    )
    .unwrap()
    .is_none());
}
