use crate::dao::in_memory::InMemoryRoomDao;
use crate::dao::RoomDao;
use crate::game::player::PlayerDetails;
use crate::game::room::settings::{RoomState, RoomType};
use crate::game::room::{
    CreateFlatRequest, RoomCommandExecution, RoomCommandExecutor, RoomData, SetFlatInfoRequest,
    UpdateFlatRequest,
};

fn owner() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(7, "alice", "hello", "hd=100");
    details
}

fn room(id: i32) -> RoomData {
    RoomData::new(
        id,
        false,
        RoomType::Private,
        7,
        "alice",
        "Old room",
        0,
        "",
        25,
        "old desc",
        "model_a",
        "default",
        "wall",
        "floor",
        false,
        true,
    )
}

#[test]
fn creates_room_and_maps_to_flat_created_outcome() {
    let dao = InMemoryRoomDao::new();

    let execution = RoomCommandExecutor::create_flat(
        &dao,
        &owner(),
        CreateFlatRequest::new("Tea Room", "model_b", 1, false),
    )
    .unwrap();

    let RoomCommandExecution::Created(room) = &execution else {
        panic!("expected created room");
    };
    assert_eq!(room.id(), 1);
    assert_eq!(room.name(), "Tea Room");
    assert_eq!(room.model_name(), "model_b");
    assert_eq!(room.state(), RoomState::Doorbell);
    assert!(!room.show_owner_name());
    assert!(execution.command_outcome().flat_created().is_some());
}

#[test]
fn returns_flat_info_for_existing_room_only() {
    let dao = InMemoryRoomDao::new();
    dao.insert_room(room(42));

    let execution = RoomCommandExecutor::get_flat_info(&dao, 42).unwrap();
    let missing = RoomCommandExecutor::get_flat_info(&dao, 99).unwrap();

    assert!(matches!(execution, RoomCommandExecution::FlatInfo(_)));
    assert!(execution.command_outcome().flat_info_packet().is_some());
    assert_eq!(missing, RoomCommandExecution::Ignored);
}

#[test]
fn deletes_room_only_with_owner_rights() {
    let dao = InMemoryRoomDao::new();
    dao.insert_room(room(42));

    let ignored = RoomCommandExecutor::delete_flat(&dao, 42, false).unwrap();
    let deleted = RoomCommandExecutor::delete_flat(&dao, 42, true).unwrap();

    assert_eq!(ignored, RoomCommandExecution::Ignored);
    assert!(dao.room(42, false).unwrap().is_none());
    assert_eq!(deleted, RoomCommandExecution::Deleted { room_id: 42 });
}

#[test]
fn updates_flat_metadata_and_preserves_short_name() {
    let dao = InMemoryRoomDao::new();
    dao.insert_room(room(42));

    let updated = RoomCommandExecutor::update_flat(
        &dao,
        UpdateFlatRequest::new(42, "Renamed", 2, false, true),
    )
    .unwrap();
    let preserved =
        RoomCommandExecutor::update_flat(&dao, UpdateFlatRequest::new(42, "x", 1, true, true))
            .unwrap();

    let RoomCommandExecution::Updated(updated) = updated else {
        panic!("expected update");
    };
    let RoomCommandExecution::Updated(preserved) = preserved else {
        panic!("expected second update");
    };
    assert_eq!(updated.name(), "Renamed");
    assert_eq!(updated.state(), RoomState::Password);
    assert!(!updated.show_owner_name());
    assert_eq!(preserved.name(), "Renamed");
    assert_eq!(preserved.state(), RoomState::Doorbell);
    assert!(preserved.show_owner_name());
}

#[test]
fn set_flat_info_updates_description_password_and_all_super_user() {
    let dao = InMemoryRoomDao::new();
    dao.insert_room(room(42));

    let updated = RoomCommandExecutor::set_flat_info(
        &dao,
        SetFlatInfoRequest::new(42, "new desc", "secret", true, true),
    )
    .unwrap();
    let preserved = RoomCommandExecutor::set_flat_info(
        &dao,
        SetFlatInfoRequest::new(42, "x", "door", false, true),
    )
    .unwrap();

    let RoomCommandExecution::Updated(updated) = updated else {
        panic!("expected info update");
    };
    let RoomCommandExecution::Updated(preserved) = preserved else {
        panic!("expected second info update");
    };
    assert_eq!(updated.description(), "new desc");
    assert_eq!(updated.password(), "secret");
    assert!(updated.has_all_super_user());
    assert_eq!(preserved.description(), "new desc");
    assert_eq!(preserved.password(), "door");
    assert!(!preserved.has_all_super_user());
}

#[test]
fn ignores_mutations_without_rights_or_missing_room() {
    let dao = InMemoryRoomDao::new();
    dao.insert_room(room(42));

    assert_eq!(
        RoomCommandExecutor::update_flat(
            &dao,
            UpdateFlatRequest::new(42, "Renamed", 1, false, false),
        )
        .unwrap(),
        RoomCommandExecution::Ignored
    );
    assert_eq!(
        RoomCommandExecutor::set_flat_info(
            &dao,
            SetFlatInfoRequest::new(99, "desc", "secret", false, true),
        )
        .unwrap(),
        RoomCommandExecution::Ignored
    );
    assert_eq!(dao.room(42, false).unwrap().unwrap().name(), "Old room");
}
