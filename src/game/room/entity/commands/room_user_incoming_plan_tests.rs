use super::*;
use crate::game::commands::CommandEffect;
use crate::game::room::model::Position;

fn room_user() -> RoomUser {
    let mut user = RoomUser::new(7, "alice", "hd=100", "hello", None::<String>);
    user.set_room_id(42);
    user
}

#[test]
fn plans_status_reset_and_update_effects() {
    let mut user = room_user();
    user.set_afk_timer(0);

    let effects = RoomUserIncomingPlan::plan_all(
        &mut user,
        &[
            IncomingExecutionEffect::SetRoomStatus {
                key: "dance".to_owned(),
                value: String::new(),
                visible: true,
                timeout: -1,
            },
            IncomingExecutionEffect::ResetAfkTimer,
            IncomingExecutionEffect::MarkRoomNeedsUpdate,
            IncomingExecutionEffect::RemoveRoomStatus {
                key: "dance".to_owned(),
            },
        ],
    );

    assert!(effects.is_empty());
    assert!(!user.contains_status("dance"));
    assert!(user.afk_timer() > 0);
    assert!(user.needs_update());
}

#[test]
fn plans_look_rotation_when_room_user_can_turn() {
    let mut user = room_user();
    user.set_position(Position::with_rotation(1, 1, 0.0, 0));

    let effects =
        RoomUserIncomingPlan::plan(&mut user, &IncomingExecutionEffect::LookTo { x: 2, y: 1 });

    assert!(effects.is_empty());
    assert_eq!(user.position().rotation(), 2);
    assert!(user.needs_update());
}

#[test]
fn plans_talk_into_room_user_effect() {
    let mut user = room_user();

    let effects = RoomUserIncomingPlan::plan(
        &mut user,
        &IncomingExecutionEffect::Talk {
            mode: "CHAT".to_owned(),
            message: "hello".to_owned(),
        },
    );

    assert_eq!(
        effects,
        vec![RoomUserEffect::Chat {
            header: "CHAT".to_owned(),
            username: "alice".to_owned(),
            message: "hello".to_owned(),
        }]
    );
    assert!(user.contains_status("talk"));
}

#[test]
fn plans_command_room_status_effects() {
    let mut user = room_user();

    let effects = RoomUserIncomingPlan::plan_all(
        &mut user,
        &[
            IncomingExecutionEffect::Command(CommandEffect::SetRoomStatus {
                key: "sit".to_owned(),
                value: " 1.0".to_owned(),
                infinite: true,
                duration: -1,
            }),
            IncomingExecutionEffect::Command(CommandEffect::RemoveRoomStatus {
                key: "sit".to_owned(),
            }),
        ],
    );

    assert!(effects.is_empty());
    assert!(!user.contains_status("sit"));
}
