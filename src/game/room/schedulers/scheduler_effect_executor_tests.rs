use std::collections::VecDeque;

use super::scheduler_effect_executor::*;

fn room_user() -> RoomUser {
    let mut user = RoomUser::new(7, "alice", "hd-100", "hello", None::<String>);
    user.set_room_id(42);
    user
}

#[test]
fn applies_room_walk_state_mutations() {
    let mut user = room_user();
    user.set_position(Position::new(0, 0, 0.0));
    user.set_path(VecDeque::from([
        Position::new(1, 0, 0.0),
        Position::new(2, 0, 0.0),
    ]));
    user.set_status("sit", " 1", true, -1);

    SchedulerEffectExecutor::apply_all(
        &mut user,
        &[
            SchedulerEffect::PopPath { entity_id: 7 },
            SchedulerEffect::RemoveStatus {
                entity_id: 7,
                key: "sit".to_owned(),
            },
            SchedulerEffect::SetRotation {
                entity_id: 7,
                rotation: 2,
            },
            SchedulerEffect::SetStatus {
                entity_id: 7,
                key: "mv".to_owned(),
                value: " 2,0,0".to_owned(),
                infinite: true,
                duration: -1,
            },
            SchedulerEffect::SetNext {
                entity_id: 7,
                position: Position::with_rotation(1, 0, 0.0, 2),
            },
            SchedulerEffect::MarkNeedsUpdate { entity_id: 7 },
        ],
    );

    assert_eq!(user.path(), &VecDeque::from([Position::new(2, 0, 0.0)]));
    assert!(!user.contains_status("sit"));
    assert!(user.contains_status("mv"));
    assert_eq!(user.position().rotation(), 2);
    assert_eq!(user.next(), Some(Position::with_rotation(1, 0, 0.0, 2)));
    assert!(user.needs_update());
}

#[test]
fn applies_user_status_tick_mutations() {
    let mut user = room_user();
    user.set_position(Position::with_rotation(1, 1, 0.0, 4));
    user.set_time_until_next_drink(2);
    user.set_status("talk", "", false, 3);

    SchedulerEffectExecutor::apply_all(
        &mut user,
        &[
            SchedulerEffect::SetHeadRotation {
                entity_id: 7,
                rotation: 4,
            },
            SchedulerEffect::SetLookResetTime {
                entity_id: 7,
                ticks: -1,
            },
            SchedulerEffect::SetTimeUntilNextDrink {
                entity_id: 7,
                ticks: 1,
            },
            SchedulerEffect::TickStatus {
                entity_id: 7,
                key: "talk".to_owned(),
            },
            SchedulerEffect::SetStatus {
                entity_id: 7,
                key: "drink".to_owned(),
                value: String::new(),
                infinite: false,
                duration: -1,
            },
            SchedulerEffect::RemoveStatus {
                entity_id: 7,
                key: "drink".to_owned(),
            },
        ],
    );

    assert_eq!(user.position().head_rotation(), 4);
    assert_eq!(user.look_reset_time(), -1);
    assert_eq!(user.time_until_next_drink(), 1);
    assert_eq!(user.status("talk").unwrap().duration(), 2);
    assert!(!user.contains_status("drink"));
}

#[test]
fn stop_walking_mutates_user_and_returns_room_user_effects() {
    let mut user = room_user();
    user.set_walking(true);
    user.set_status("mv", " 1,0,0", true, -1);
    user.set_current_item_id(Some(12));

    let effects =
        SchedulerEffectExecutor::apply(&mut user, &SchedulerEffect::StopWalking { entity_id: 7 });

    assert_eq!(
        effects,
        vec![RoomUserEffect::TriggerCurrentItem { item_id: Some(12) }]
    );
    assert!(!user.is_walking());
    assert!(!user.contains_status("mv"));
    assert!(user.needs_update());
}

#[test]
fn ignores_effects_for_other_users_or_other_boundaries() {
    let mut user = room_user();

    let effects = SchedulerEffectExecutor::apply_all(
        &mut user,
        &[
            SchedulerEffect::SetStatus {
                entity_id: 9,
                key: "mv".to_owned(),
                value: String::new(),
                infinite: true,
                duration: -1,
            },
            SchedulerEffect::WalkTo {
                entity_id: 7,
                x: 1,
                y: 1,
            },
            SchedulerEffect::SendStatus(vec![7]),
        ],
    );

    assert!(effects.is_empty());
    assert!(!user.contains_status("mv"));
}
