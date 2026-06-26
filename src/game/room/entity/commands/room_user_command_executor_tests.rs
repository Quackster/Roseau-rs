use crate::game::commands::CommandEffect;
use crate::game::room::entity::{RoomUser, RoomUserCommandExecutor, RoomUserEffect};
use crate::game::room::model::Position;
use crate::messages::IncomingExecutionEffect;

fn room_user() -> RoomUser {
    let mut user = RoomUser::new(7, "alice", "hd=100", "hello", None::<String>);
    user.set_room_id(42);
    user
}

#[test]
fn applies_status_and_update_effects_to_room_user() {
    let mut user = room_user();

    RoomUserCommandExecutor::apply_all(
        &mut user,
        &[
            IncomingExecutionEffect::SetRoomStatus {
                key: "dance".to_owned(),
                value: String::new(),
                visible: true,
                timeout: -1,
            },
            IncomingExecutionEffect::MarkRoomNeedsUpdate,
            IncomingExecutionEffect::RemoveRoomStatus {
                key: "dance".to_owned(),
            },
        ],
    );

    assert!(!user.contains_status("dance"));
    assert!(user.needs_update());
}

#[test]
fn resets_afk_timer_for_incoming_room_user_effect() {
    let mut user = room_user();
    user.set_afk_timer(0);

    RoomUserCommandExecutor::apply(&mut user, &IncomingExecutionEffect::ResetAfkTimer);

    assert!(user.afk_timer() > 0);
}

#[test]
fn rotates_user_for_look_to_when_java_guards_allow_it() {
    let mut user = room_user();
    user.set_position(Position::with_rotation(1, 1, 0.0, 0));

    RoomUserCommandExecutor::apply(&mut user, &IncomingExecutionEffect::LookTo { x: 2, y: 1 });

    assert_eq!(user.position().rotation(), 2);
    assert!(user.needs_update());
}

#[test]
fn ignores_look_to_while_user_is_sitting() {
    let mut user = room_user();
    user.set_position(Position::with_rotation(1, 1, 0.0, 0));
    user.set_status("sit", " 1", true, -1);

    RoomUserCommandExecutor::apply(&mut user, &IncomingExecutionEffect::LookTo { x: 2, y: 1 });

    assert_eq!(user.position().rotation(), 0);
    assert!(!user.needs_update());
}

#[test]
fn emits_chat_effect_for_talk() {
    let mut user = room_user();

    let effects = RoomUserCommandExecutor::apply(
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
}

#[test]
fn applies_java_talk_and_emote_statuses_for_chat() {
    let mut user = room_user();

    RoomUserCommandExecutor::apply(
        &mut user,
        &IncomingExecutionEffect::Talk {
            mode: "CHAT".to_owned(),
            message: "hello :)".to_owned(),
        },
    );

    let talk = user.status("talk").unwrap();
    let gest = user.status("gest").unwrap();

    assert_eq!(talk.value(), "");
    assert_eq!(talk.duration(), 4);
    assert_eq!(gest.value(), " sml");
    assert_eq!(gest.duration(), 5);
    assert!(user.needs_update());
}

#[test]
fn caps_java_talk_status_duration_at_five_ticks() {
    let mut user = room_user();

    RoomUserCommandExecutor::apply(
        &mut user,
        &IncomingExecutionEffect::Talk {
            mode: "SHOUT".to_owned(),
            message: "hello there".to_owned(),
        },
    );

    assert_eq!(user.status("talk").unwrap().duration(), 5);
}

#[test]
fn emits_targeted_whisper_effect_for_whisper_talk() {
    let mut user = room_user();

    let effects = RoomUserCommandExecutor::apply(
        &mut user,
        &IncomingExecutionEffect::Talk {
            mode: "WHISPER".to_owned(),
            message: "bob hello there".to_owned(),
        },
    );

    assert_eq!(
        effects,
        vec![RoomUserEffect::Whisper {
            username: "alice".to_owned(),
            target_username: Some("bob".to_owned()),
            message: "hello there".to_owned(),
        }]
    );
}

#[test]
fn does_not_apply_talk_statuses_for_whisper() {
    let mut user = room_user();

    RoomUserCommandExecutor::apply(
        &mut user,
        &IncomingExecutionEffect::Talk {
            mode: "WHISPER".to_owned(),
            message: "bob :)".to_owned(),
        },
    );

    assert!(user.status("talk").is_none());
    assert!(user.status("gest").is_none());
    assert!(!user.needs_update());
}

#[test]
fn preserves_single_argument_whisper_as_self_echo_text() {
    let mut user = room_user();

    let effects = RoomUserCommandExecutor::apply(
        &mut user,
        &IncomingExecutionEffect::Talk {
            mode: "WHISPER".to_owned(),
            message: "bob ".to_owned(),
        },
    );

    assert_eq!(
        effects,
        vec![RoomUserEffect::Whisper {
            username: "alice".to_owned(),
            target_username: None,
            message: "bob ".to_owned(),
        }]
    );
}

#[test]
fn applies_colon_command_room_user_effects() {
    let mut user = room_user();

    RoomUserCommandExecutor::apply_all(
        &mut user,
        &[
            IncomingExecutionEffect::Command(CommandEffect::SetRoomStatus {
                key: "sit".to_owned(),
                value: " 1".to_owned(),
                infinite: true,
                duration: -1,
            }),
            IncomingExecutionEffect::Command(CommandEffect::MarkRoomNeedsUpdate),
        ],
    );

    assert!(user.contains_status("sit"));
    assert!(user.needs_update());
}
