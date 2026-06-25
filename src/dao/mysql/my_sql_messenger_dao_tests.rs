use std::cell::RefCell;
use std::collections::VecDeque;

use super::my_sql_messenger_dao::*;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter, SqlRow, SqlValue};

#[derive(Debug, Default)]
struct RecordingExecutor {
    plans: RefCell<Vec<SqlExecutionPlan>>,
    results: RefCell<VecDeque<Result<SqlExecutionResult, DaoError>>>,
}

impl RecordingExecutor {
    fn push_result(&self, result: SqlExecutionResult) {
        self.results.borrow_mut().push_back(Ok(result));
    }

    fn plans(&self) -> Vec<SqlExecutionPlan> {
        self.plans.borrow().clone()
    }
}

impl SqlExecutor for RecordingExecutor {
    fn execute(&self, plan: SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        self.plans.borrow_mut().push(plan);
        self.results
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| Err(DaoError::new("missing queued SQL result")))
    }
}

fn friendship_row(sender: i32, receiver: i32) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("sender", SqlValue::Integer(sender)),
        ("receiver", SqlValue::Integer(receiver)),
    ])
}

fn request_row(from_id: i32, to_id: i32) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(2)),
        ("to_id", SqlValue::Integer(to_id)),
        ("from_id", SqlValue::Integer(from_id)),
    ])
}

fn message_row(id: i32, from_id: i32, to_id: i32) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(id)),
        ("from_id", SqlValue::Integer(from_id)),
        ("to_id", SqlValue::Integer(to_id)),
        ("time_sent", SqlValue::Long(1234)),
        ("message", SqlValue::Text("hello".to_owned())),
        ("unread", SqlValue::Integer(1)),
    ])
}

#[test]
fn loads_friends_requests_and_unread_messages() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([
        friendship_row(7, 10),
        friendship_row(11, 7),
    ]));
    executor.push_result(SqlExecutionResult::rows([request_row(20, 7)]));
    executor.push_result(SqlExecutionResult::rows([message_row(5, 1, 7)]));
    let dao = MySqlMessengerDao::new(executor, 1234);

    assert_eq!(
        dao.friends(7)
            .unwrap()
            .iter()
            .map(MessengerUser::user_id)
            .collect::<Vec<_>>(),
        vec![10, 11]
    );
    assert_eq!(dao.requests(7).unwrap()[0].user_id(), 20);
    assert_eq!(dao.unread_messages(7).unwrap()[0].id(), 5);

    let plans = dao.executor().plans();
    assert_eq!(
        plans[0].sql(),
        "SELECT * FROM messenger_friendships WHERE sender = ? OR receiver = ?"
    );
    assert_eq!(
        plans[1].sql(),
        "SELECT * FROM messenger_requests WHERE to_id = ?"
    );
    assert_eq!(
        plans[2].sql(),
        "SELECT * FROM messenger_messages WHERE to_id = ? AND unread = ?"
    );
}

#[test]
fn creates_request_only_when_no_bidirectional_request_exists() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([]));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    executor.push_result(SqlExecutionResult::rows([request_row(1, 2)]));
    let dao = MySqlMessengerDao::new(executor, 1234);

    assert!(dao.new_request(1, 2).unwrap());
    assert!(!dao.new_request(1, 2).unwrap());

    let plans = dao.executor().plans();
    assert_eq!(plans.len(), 3);
    assert_eq!(plans[0].kind(), SqlExecutionKind::ReadRows);
    assert_eq!(
        plans[0].sql(),
        "SELECT * FROM messenger_requests WHERE (to_id = ? AND from_id = ?) OR (from_id = ? AND to_id = ?) LIMIT 1"
    );
    assert_eq!(plans[1].kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plans[1].sql(),
        "INSERT INTO messenger_requests (from_id, to_id) VALUES (?, ?)"
    );
}

#[test]
fn creates_friend_and_removes_relationships_as_mutations() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let dao = MySqlMessengerDao::new(executor, 1234);

    assert!(dao.new_friend(1, 2).unwrap());
    dao.remove_friend(2, 1).unwrap();
    dao.remove_request(1, 2).unwrap();

    let plans = dao.executor().plans();
    assert_eq!(
        plans[0].sql(),
        "INSERT INTO messenger_friendships (sender, receiver) VALUES (?, ?)"
    );
    assert_eq!(
        plans[1].sql(),
        "DELETE FROM messenger_friendships WHERE (sender = ? AND receiver = ?) OR (receiver = ? AND sender = ?)"
    );
    assert_eq!(
        plans[2].sql(),
        "DELETE FROM messenger_requests WHERE from_id = ? AND to_id = ?"
    );
}

#[test]
fn creates_message_with_timestamp_and_generated_id() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::insert_id(42));
    let dao = MySqlMessengerDao::new(executor, 5555);

    assert_eq!(dao.new_message(1, 2, "hello").unwrap(), 42);

    let plans = dao.executor().plans();
    assert_eq!(plans[0].kind(), SqlExecutionKind::InsertReturningId);
    assert_eq!(
        plans[0].sql(),
        "INSERT INTO messenger_messages (from_id, to_id, time_sent, message, unread) VALUES (?, ?, ?, ?, ?)"
    );
    assert_eq!(
        plans[0].parameters(),
        &[
            SqlParameter::Integer(1),
            SqlParameter::Integer(2),
            SqlParameter::Long(5555),
            SqlParameter::Text("hello".to_owned()),
            SqlParameter::Bool(true),
        ]
    );
}

#[test]
fn marks_messages_read_and_validates_result_kinds() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let dao = MySqlMessengerDao::new(executor, 1234);

    dao.mark_message_read(9).unwrap();
    assert_eq!(
        dao.friends(7).unwrap_err().message(),
        "SQL execution kind ReadRows returned affected rows result"
    );

    let plans = dao.executor().plans();
    assert_eq!(
        plans[0].sql(),
        "UPDATE messenger_messages SET unread = ? WHERE id = ?"
    );
}
