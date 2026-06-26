use super::*;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter};

#[test]
fn maps_mark_read_effect_to_update_plan() {
    let plans = MessengerCommandQueries::plan(
        &IncomingExecutionEffect::MarkMessengerMessageRead { message_id: 77 },
        5,
        1234,
    );

    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plans[0].sql(),
        "UPDATE messenger_messages SET unread = ? WHERE id = ?"
    );
    assert_eq!(
        plans[0].parameters(),
        &[SqlParameter::Bool(false), SqlParameter::Integer(77)]
    );
}

#[test]
fn maps_send_message_effect_to_one_insert_per_receiver() {
    let plans = MessengerCommandQueries::plan(
        &IncomingExecutionEffect::SendMessengerMessage {
            receiver_ids: vec![8, 9],
            message: "hello".to_owned(),
        },
        5,
        1234,
    );

    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plans[0].sql(),
        "INSERT INTO messenger_messages (from_id, to_id, time_sent, message, unread) VALUES (?, ?, ?, ?, ?)"
    );
    assert_eq!(
        plans[0].parameters(),
        &[
            SqlParameter::Integer(5),
            SqlParameter::Integer(8),
            SqlParameter::Long(1234),
            SqlParameter::Text("hello".to_owned()),
            SqlParameter::Bool(true),
        ]
    );
    assert_eq!(
        plans[1].parameters(),
        &[
            SqlParameter::Integer(5),
            SqlParameter::Integer(9),
            SqlParameter::Long(1234),
            SqlParameter::Text("hello".to_owned()),
            SqlParameter::Bool(true),
        ]
    );
}

#[test]
fn ignores_non_messenger_persistence_effects() {
    assert!(MessengerCommandQueries::plan(&IncomingExecutionEffect::GoAway, 5, 1234).is_empty());
}
