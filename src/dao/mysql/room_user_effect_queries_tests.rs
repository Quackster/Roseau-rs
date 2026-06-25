use super::room_user_effect_queries::*;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter};
use crate::game::player::{PlayerDetails, PlayerSession};

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details
}

fn manager() -> PlayerManager {
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(70, 30000, details(7, "alice")));
    manager.insert(PlayerSession::new(80, 30000, details(8, "bob")));
    manager
}

#[test]
fn maps_delivered_whisper_effect_to_java_chatlog_insert() {
    let plan = RoomUserEffectQueries::delivered_whisper_chatlog_plan(
        &RoomUserEffect::Whisper {
            username: "alice".to_owned(),
            target_username: Some("Bob".to_owned()),
            message: "secret".to_owned(),
        },
        7,
        42,
        1234,
        &manager(),
    )
    .unwrap();

    assert_eq!(plan.kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plan.sql(),
        "INSERT INTO room_chatlogs (user, room_id, timestamp, message_type, message) VALUES (?, ?, ?, ?, ?)"
    );
    assert_eq!(
        plan.parameters(),
        &[
            SqlParameter::Text("alice".to_owned()),
            SqlParameter::Integer(42),
            SqlParameter::Long(1234),
            SqlParameter::Integer(2),
            SqlParameter::Text("(to: Bob) secret".to_owned()),
        ]
    );
}

#[test]
fn skips_whisper_chatlog_when_target_is_missing_self_or_unspecified() {
    let manager = manager();

    assert_eq!(
        RoomUserEffectQueries::delivered_whisper_chatlog_plan(
            &RoomUserEffect::Whisper {
                username: "alice".to_owned(),
                target_username: Some("nobody".to_owned()),
                message: "secret".to_owned(),
            },
            7,
            42,
            1234,
            &manager,
        ),
        None
    );
    assert_eq!(
        RoomUserEffectQueries::delivered_whisper_chatlog_plan(
            &RoomUserEffect::Whisper {
                username: "alice".to_owned(),
                target_username: Some("alice".to_owned()),
                message: "secret".to_owned(),
            },
            7,
            42,
            1234,
            &manager,
        ),
        None
    );
    assert_eq!(
        RoomUserEffectQueries::delivered_whisper_chatlog_plan(
            &RoomUserEffect::Whisper {
                username: "alice".to_owned(),
                target_username: None,
                message: "secret".to_owned(),
            },
            7,
            42,
            1234,
            &manager,
        ),
        None
    );
}
