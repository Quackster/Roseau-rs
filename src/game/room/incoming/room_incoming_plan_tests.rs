use super::room_incoming_plan::*;
use crate::dao::in_memory::InMemoryRoomDao;
use crate::dao::RoomDao;
use crate::game::room::settings::{RoomState, RoomType};
use crate::game::room::RoomData;

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
fn plans_create_and_get_flat_info_effects() {
    let dao = InMemoryRoomDao::new();

    let executions = RoomIncomingPlan::plan_all(
        &[
            IncomingExecutionEffect::CreateFlat {
                floor: "floor1".to_owned(),
                room_name: "Tea Room".to_owned(),
                room_model: "model_b".to_owned(),
                state: 1,
                show_owner_name: false,
            },
            IncomingExecutionEffect::GetFlatInfo { room_id: 1 },
        ],
        &dao,
        &owner(),
        true,
    )
    .unwrap();

    assert_eq!(executions.len(), 2);
    let RoomCommandExecution::Created(created) = &executions[0] else {
        panic!("expected created room");
    };
    let RoomCommandExecution::FlatInfo(info) = &executions[1] else {
        panic!("expected flat info");
    };
    assert_eq!(created.name(), "Tea Room");
    assert_eq!(info.id(), created.id());
}

#[test]
fn plans_update_set_info_and_delete_with_rights() {
    let dao = InMemoryRoomDao::new();
    dao.insert_room(room(42));

    let executions = RoomIncomingPlan::plan_all(
        &[
            IncomingExecutionEffect::UpdateFlat {
                room_id: 42,
                room_name: "Renamed".to_owned(),
                state: 2,
                show_owner_name: false,
            },
            IncomingExecutionEffect::SetFlatInfo {
                room_id: 42,
                description: "new desc".to_owned(),
                password: "secret".to_owned(),
                all_super_user: true,
            },
            IncomingExecutionEffect::DeleteFlat { room_id: 42 },
        ],
        &dao,
        &owner(),
        true,
    )
    .unwrap();

    assert_eq!(executions.len(), 3);
    let RoomCommandExecution::Updated(updated) = &executions[0] else {
        panic!("expected update");
    };
    let RoomCommandExecution::Updated(info) = &executions[1] else {
        panic!("expected flat-info update");
    };
    assert_eq!(updated.name(), "Renamed");
    assert_eq!(updated.state(), RoomState::Password);
    assert_eq!(info.description(), "new desc");
    assert_eq!(executions[2], RoomCommandExecution::Deleted { room_id: 42 });
    assert!(dao.room(42, false).unwrap().is_none());
}

#[test]
fn ignores_mutations_without_owner_rights() {
    let dao = InMemoryRoomDao::new();
    dao.insert_room(room(42));

    let executions = RoomIncomingPlan::plan_all(
        &[
            IncomingExecutionEffect::UpdateFlat {
                room_id: 42,
                room_name: "Renamed".to_owned(),
                state: 1,
                show_owner_name: false,
            },
            IncomingExecutionEffect::SetFlatInfo {
                room_id: 42,
                description: "desc".to_owned(),
                password: "secret".to_owned(),
                all_super_user: false,
            },
            IncomingExecutionEffect::DeleteFlat { room_id: 42 },
        ],
        &dao,
        &owner(),
        false,
    )
    .unwrap();

    assert_eq!(
        executions,
        vec![
            RoomCommandExecution::Ignored,
            RoomCommandExecution::Ignored,
            RoomCommandExecution::Ignored,
        ]
    );
    assert_eq!(dao.room(42, false).unwrap().unwrap().name(), "Old room");
}

#[test]
fn ignores_unrelated_incoming_effects() {
    assert!(RoomIncomingPlan::plan(
        &IncomingExecutionEffect::GoAway,
        &InMemoryRoomDao::new(),
        &owner(),
        true,
    )
    .unwrap()
    .is_empty());
}
