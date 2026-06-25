use std::collections::VecDeque;

use crate::game::room::entity::{RoomUser, RoomUserEffect};
use crate::game::room::model::Position;
use crate::messages::outgoing::{Status, Users};
use crate::messages::OutgoingMessage;
use crate::settings::CARRY_DRINK_INTERVAL_TICKS;

fn room_user() -> RoomUser {
    let mut user = RoomUser::new(7, "alice", "hd-100", "hello", Some("pool"));
    user.set_room_id(33);
    user
}

#[test]
fn resets_runtime_state_on_dispose() {
    let mut user = room_user();
    user.set_status("sit", " 1", true, -1);
    user.set_path(VecDeque::from([Position::new(1, 1, 0.0)]));
    user.set_walking(true);
    user.set_dance_id(2);

    user.dispose();

    assert!(user.statuses().is_empty());
    assert!(user.path().is_empty());
    assert_eq!(user.position(), Position::new(0, 0, 0.0));
    assert_eq!(user.goal(), Some(Position::new(0, 0, 0.0)));
    assert!(!user.is_walking());
    assert_eq!(user.dance_id(), 0);
    assert_eq!(user.time_until_next_drink(), -1);
}

#[test]
fn starts_walking_when_room_and_path_are_available() {
    let mut user = room_user();
    let path = VecDeque::from([Position::new(1, 0, 0.0), Position::new(2, 0, 0.0)]);

    assert!(user.walk_to(2, 0, path.clone()));
    assert!(user.is_walking());
    assert_eq!(user.goal(), Some(Position::new(2, 0, 0.0)));
    assert_eq!(user.path(), &path);
}

#[test]
fn refuses_walking_without_room_permission_or_path() {
    let mut no_room = RoomUser::new(7, "alice", "hd-100", "hello", None::<String>);
    assert!(!no_room.walk_to(1, 1, VecDeque::from([Position::new(1, 1, 0.0)])));

    let mut blocked = room_user();
    blocked.set_can_walk(false);
    assert!(!blocked.walk_to(1, 1, VecDeque::from([Position::new(1, 1, 0.0)])));

    let mut empty_path = room_user();
    assert!(!empty_path.walk_to(1, 1, VecDeque::new()));
}

#[test]
fn stops_walking_and_returns_current_item_effect() {
    let mut user = room_user();
    user.set_walking(true);
    user.set_status("mv", " 1,1,0", true, -1);
    user.set_current_item_id(Some(42));

    assert_eq!(
        user.stop_walking(),
        vec![RoomUserEffect::TriggerCurrentItem { item_id: Some(42) }]
    );
    assert!(!user.is_walking());
    assert!(!user.contains_status("mv"));
    assert!(user.needs_update());
}

#[test]
fn go_away_walks_to_door_and_kicks_when_stopped() {
    let mut user = room_user();
    user.set_position(Position::new(1, 1, 0.0));
    let path = VecDeque::from([Position::new(2, 1, 0.0), Position::new(3, 1, 0.0)]);

    let effects = user.go_away(Position::new(3, 1, 0.0), path.clone());

    assert!(effects.is_empty());
    assert!(user.is_walking());
    assert!(user.kick_when_stop());
    assert_eq!(user.goal(), Some(Position::new(3, 1, 0.0)));
    assert_eq!(user.path(), &path);
    assert_eq!(user.stop_walking(), vec![RoomUserEffect::Kick]);
}

#[test]
fn go_away_kicks_immediately_at_door_or_when_walk_fails() {
    let mut at_door = room_user();
    at_door.set_position(Position::new(3, 1, 0.0));

    assert_eq!(
        at_door.go_away(Position::new(3, 1, 0.0), VecDeque::new()),
        vec![RoomUserEffect::Kick]
    );

    let mut no_path = room_user();
    no_path.set_position(Position::new(1, 1, 0.0));
    assert_eq!(
        no_path.go_away(Position::new(3, 1, 0.0), VecDeque::new()),
        vec![RoomUserEffect::Kick]
    );
}

#[test]
fn splash_from_pool_lift_warps_user_and_starts_exit_walk() {
    let mut user = room_user();
    user.set_can_walk(false);
    let exit_path = VecDeque::from([Position::new(18, 18, 0.0), Position::new(18, 19, 0.0)]);

    let effects = user.splash_from_pool_lift(Position::new(17, 18, 0.5), exit_path.clone());

    assert_eq!(
        effects,
        vec![RoomUserEffect::ShowProgram(vec![
            "BIGSPLASH".to_owned(),
            "POSITION".to_owned(),
            "17,18".to_owned(),
        ])]
    );
    assert_eq!(user.position(), Position::new(17, 18, 0.5));
    assert!(user.contains_status("swim"));
    assert!(user.can_walk());
    assert_eq!(user.goal(), Some(Position::new(18, 19, 0.0)));
    assert_eq!(user.path(), &exit_path);
}

#[test]
fn look_towards_updates_head_rotation_when_not_walking() {
    let mut user = room_user();
    user.set_position(Position::with_rotation(5, 5, 0.0, 2));

    user.look_towards(Position::new(5, 6, 0.0));

    assert_eq!(user.position().head_rotation(), 3);
    assert!(user.needs_update());
}

#[test]
fn carry_status_sets_drink_interval_and_optional_update() {
    let mut user = room_user();
    user.set_status_with_update("carryd", "2", false, 5, true);

    assert_eq!(user.time_until_next_drink(), CARRY_DRINK_INTERVAL_TICKS);
    assert!(user.contains_status("carryd"));
    assert!(user.needs_update());
}

#[test]
fn builds_status_and_users_composer_entities() {
    let mut user = room_user();
    user.set_position(Position::with_rotation(1, 2, 3.5, 4));
    user.set_status("sit", " 3", true, -1);

    let mut status = Status::new([user.status_entity()]).compose();
    let mut users = Users::new([user.user_entry()]).compose();

    assert_eq!(status.get(), "#STATUS \ralice 1,2,3.5,4,4/sit 3/##");
    assert_eq!(users.get(), "#USERS\r  alice hd-100 1 2 3.5 hello pool##");
}
