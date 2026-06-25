use super::user_status_event::*;
use crate::game::room::entity::RoomUserStatus;

#[test]
fn ticks_expiring_statuses_every_second_interval() {
    let mut event = UserStatusEvent::new();
    let user = RoomUserTickState::new(7).with_status(RoomUserStatus::new("wave", "", false, 1));

    assert_eq!(
        event.tick(&[user.clone()]),
        vec![
            SchedulerEffect::TickStatus {
                entity_id: 7,
                key: "wave".to_owned()
            },
            SchedulerEffect::RemoveStatus {
                entity_id: 7,
                key: "wave".to_owned()
            },
            SchedulerEffect::MarkNeedsUpdate { entity_id: 7 }
        ]
    );
    assert!(event.tick(&[user]).is_empty());
}

#[test]
fn ticks_finite_statuses_that_have_not_expired() {
    let mut event = UserStatusEvent::new();
    let user = RoomUserTickState::new(8).with_status(RoomUserStatus::new("talk", "", false, 3));

    assert_eq!(
        event.tick(&[user]),
        vec![SchedulerEffect::TickStatus {
            entity_id: 8,
            key: "talk".to_owned()
        }]
    );
}

#[test]
fn converts_carried_drink_to_drink_animation_when_allowed() {
    let mut event = UserStatusEvent::new();
    let user = RoomUserTickState::new(9)
        .time_until_next_drink(0)
        .with_status(RoomUserStatus::new("carryd", "2", false, 5));

    assert_eq!(
        event.tick(&[user]),
        vec![
            SchedulerEffect::RemoveStatus {
                entity_id: 9,
                key: "carryd".to_owned()
            },
            SchedulerEffect::SetStatus {
                entity_id: 9,
                key: "drink".to_owned(),
                value: String::new(),
                infinite: false,
                duration: -1
            },
            SchedulerEffect::RemoveStatus {
                entity_id: 9,
                key: "drink".to_owned()
            },
            SchedulerEffect::SetStatus {
                entity_id: 9,
                key: "carryd".to_owned(),
                value: "2".to_owned(),
                infinite: false,
                duration: 5
            }
        ]
    );
}

#[test]
fn resets_look_rotation_when_timer_reaches_zero() {
    let mut event = UserStatusEvent::new();
    let user = RoomUserTickState::new(11)
        .look_reset_time(0)
        .rotations(6, 2);

    assert_eq!(
        event.tick(&[user]),
        vec![
            SchedulerEffect::SetHeadRotation {
                entity_id: 11,
                rotation: 6
            },
            SchedulerEffect::SetLookResetTime {
                entity_id: 11,
                ticks: -1
            },
            SchedulerEffect::MarkNeedsUpdate { entity_id: 11 }
        ]
    );
}
