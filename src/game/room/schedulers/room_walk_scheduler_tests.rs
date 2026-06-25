use std::collections::VecDeque;

use super::*;
use crate::game::item::ItemDefinition;
use crate::game::room::model::RoomModel;

fn mapping() -> RoomMapping {
    RoomMapping::new(RoomModel::new("model_a", "000", 0, 0, 0, 0, false, false).unwrap())
}

fn chair() -> Item {
    Item::new(
        1,
        1,
        1,
        "1",
        0,
        0.0,
        0,
        ItemDefinition::new(1, "chair", "", 1, 1, 1.0, "SFC", "", "", ""),
        "",
        None,
    )
    .unwrap()
}

#[test]
fn emits_movement_effects_for_next_path_step() {
    let mut mapping = mapping();
    let item = chair();
    mapping.regenerate_collision_maps([item.clone()]);
    let entity = RoomWalkEntity::new(7, Position::new(0, 0, 0.0))
        .walking(true)
        .path(VecDeque::from([Position::new(1, 0, 0.0)]));

    let effects = RoomWalkScheduler::tick(&[entity], &mapping, &[item], &[], false);

    assert!(effects.contains(&SchedulerEffect::SetLookResetTime {
        entity_id: 7,
        ticks: -1,
    }));
    assert!(effects.contains(&SchedulerEffect::PopPath { entity_id: 7 }));
    assert!(effects.contains(&SchedulerEffect::RemoveStatus {
        entity_id: 7,
        key: "lay".to_owned(),
    }));
    assert!(effects.contains(&SchedulerEffect::SetRotation {
        entity_id: 7,
        rotation: 2,
    }));
    assert!(effects.contains(&SchedulerEffect::SetStatus {
        entity_id: 7,
        key: "mv".to_owned(),
        value: " 1,0,1".to_owned(),
        infinite: true,
        duration: -1,
    }));
    assert_eq!(effects.last(), Some(&SchedulerEffect::SendStatus(vec![7])));
}

#[test]
fn moves_to_existing_next_before_processing_path() {
    let mapping = mapping();
    let entity = RoomWalkEntity::new(7, Position::new(0, 0, 0.0))
        .walking(true)
        .with_next(Some(Position::with_rotation(1, 0, 2.0, 2)))
        .path(VecDeque::from([Position::new(2, 0, 0.0)]));

    let effects = RoomWalkScheduler::tick(&[entity], &mapping, &[], &[], false);

    assert!(effects.contains(&SchedulerEffect::MoveTo {
        entity_id: 7,
        position: Position::with_rotation(1, 0, 2.0, 2),
    }));
    assert!(effects.contains(&SchedulerEffect::UpdateHeight {
        entity_id: 7,
        height: 2.0,
    }));
}

#[test]
fn clears_invalid_path_and_requests_repath_to_goal() {
    let mapping = mapping();
    let blocker = RoomOccupant::new(8, Position::new(1, 0, 0.0), None);
    let entity = RoomWalkEntity::new(7, Position::new(0, 0, 0.0))
        .walking(true)
        .with_goal(Some(Position::new(2, 0, 0.0)))
        .path(VecDeque::from([Position::new(1, 0, 0.0)]));

    let effects = RoomWalkScheduler::tick(&[entity], &mapping, &[], &[blocker], false);

    assert!(effects.contains(&SchedulerEffect::ClearPath { entity_id: 7 }));
    assert!(effects.contains(&SchedulerEffect::WalkTo {
        entity_id: 7,
        x: 2,
        y: 0,
    }));
}

#[test]
fn stops_walking_when_path_is_empty() {
    let mapping = mapping();
    let entity = RoomWalkEntity::new(7, Position::new(0, 0, 0.0))
        .walking(true)
        .current_item_id(Some(12));

    let effects = RoomWalkScheduler::tick(&[entity], &mapping, &[], &[], false);

    assert!(effects.contains(&SchedulerEffect::StopWalking { entity_id: 7 }));
    assert!(effects.contains(&SchedulerEffect::TriggerCurrentItem {
        entity_id: 7,
        item_id: Some(12),
    }));
}
