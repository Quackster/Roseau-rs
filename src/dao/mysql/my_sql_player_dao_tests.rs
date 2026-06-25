use std::cell::RefCell;
use std::collections::VecDeque;

use crate::dao::mysql::{
    MySqlPlayerDao, PlayerPasswordQueries, SqlExecutionKind, SqlExecutionPlan, SqlExecutionResult,
    SqlExecutor, SqlParameter, SqlRow, SqlValue,
};
use crate::dao::{CreatePlayer, DaoError, PlayerDao};
use crate::game::player::{PasswordHasher, PlayerDetails};

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

fn dao(executor: RecordingExecutor) -> MySqlPlayerDao<RecordingExecutor> {
    MySqlPlayerDao::new(
        executor,
        PlayerPasswordQueries::new(PasswordHasher::new(4)),
        100,
        "Welcome",
        1234,
    )
}

fn create_player() -> CreatePlayer {
    CreatePlayer::new(
        "alice",
        "secret",
        "alice@example.test",
        "hello",
        "hd-100",
        1,
        "F",
        "1990-01-01",
    )
}

fn user_row(password: impl Into<String>) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(7)),
        ("username", SqlValue::Text("alice".to_owned())),
        ("password", SqlValue::Text(password.into())),
        ("rank", SqlValue::Integer(4)),
        ("mission", SqlValue::Text("hello".to_owned())),
        ("figure", SqlValue::Text("hd-100".to_owned())),
        ("pool_figure", SqlValue::Text("pool".to_owned())),
        ("email", SqlValue::Text("alice@example.test".to_owned())),
        ("credits", SqlValue::Integer(55)),
        ("sex", SqlValue::Text("F".to_owned())),
        ("country", SqlValue::Text("UK".to_owned())),
        ("badge", SqlValue::Text("ADM".to_owned())),
        ("birthday", SqlValue::Text("1990-01-01".to_owned())),
        ("join_date", SqlValue::Long(1000)),
        ("last_online", SqlValue::Long(2000)),
        ("personal_greeting", SqlValue::Text("welcome".to_owned())),
        ("tickets", SqlValue::Integer(8)),
    ])
}

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        7,
        "alice",
        "changed",
        "hd-200",
        "pool",
        "alice@example.test",
        4,
        55,
        "F",
        "UK",
        "ADM",
        "1990-01-01",
        2000,
        "welcome",
        8,
    );
    details.set_password("new-secret");
    details
}

#[test]
fn creates_player_with_hashed_password_and_runtime_defaults() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::insert_id(7));
    let dao = dao(executor);

    dao.create_player(&create_player()).unwrap();

    let plans = dao.executor().plans();
    assert_eq!(plans.len(), 1);
    assert_eq!(plans[0].kind(), SqlExecutionKind::InsertReturningId);
    assert_eq!(
        plans[0].sql(),
        "INSERT INTO users (username, password, email, mission, figure, credits, sex, birthday, join_date, last_online, personal_greeting) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    );
    assert!(
        matches!(&plans[0].parameters()[1], SqlParameter::Text(hash) if hash.starts_with("$2a$04$"))
    );
    assert_eq!(plans[0].parameters()[5], SqlParameter::Integer(100));
    assert_eq!(plans[0].parameters()[8], SqlParameter::Long(1234));
    assert_eq!(
        plans[0].parameters()[10],
        SqlParameter::Text("Welcome".to_owned())
    );
}

#[test]
fn maps_detail_id_name_and_permission_reads() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([user_row("hash")]));
    executor.push_result(SqlExecutionResult::rows([SqlRow::new([(
        "id",
        SqlValue::Integer(7),
    )])]));
    executor.push_result(SqlExecutionResult::rows([SqlRow::new([(
        "id",
        SqlValue::Integer(7),
    )])]));
    executor.push_result(SqlExecutionResult::rows([SqlRow::new([
        ("id", SqlValue::Integer(1)),
        ("rank", SqlValue::Integer(4)),
        ("permission", SqlValue::Text("room_admin".to_owned())),
        ("inheritable", SqlValue::Integer(1)),
    ])]));
    let dao = dao(executor);

    assert_eq!(dao.details_by_id(7).unwrap().unwrap().username(), "alice");
    assert_eq!(dao.id_by_username("alice").unwrap(), Some(7));
    assert!(dao.is_name_taken("alice").unwrap());
    assert_eq!(dao.permissions().unwrap()[0].permission(), "room_admin");

    let plans = dao.executor().plans();
    assert_eq!(plans[0].sql(), "SELECT * FROM users WHERE id = ? LIMIT 1");
    assert_eq!(
        plans[1].sql(),
        "SELECT id FROM users WHERE username = ? LIMIT 1"
    );
    assert_eq!(
        plans[2].sql(),
        "SELECT id FROM users WHERE username = ? LIMIT 1"
    );
    assert_eq!(plans[3].sql(), "SELECT * FROM users_permissions");
}

#[test]
fn verifies_login_password_before_returning_authenticated_result() {
    let hasher = PasswordHasher::new(4);
    let password_hash = hasher.hash_password("secret").unwrap();
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([user_row(password_hash)]));
    let dao = MySqlPlayerDao::new(
        executor,
        PlayerPasswordQueries::new(hasher),
        100,
        "Welcome",
        1234,
    );

    let result = dao.login("alice", "secret").unwrap().unwrap();

    assert!(result.authenticated);
    assert_eq!(result.details.username(), "alice");
    assert_eq!(
        dao.executor().plans()[0].sql(),
        "SELECT * FROM users WHERE username = ? LIMIT 1"
    );
}

#[test]
fn rejects_missing_or_wrong_login_passwords() {
    let hasher = PasswordHasher::new(4);
    let password_hash = hasher.hash_password("secret").unwrap();
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([]));
    executor.push_result(SqlExecutionResult::rows([user_row(password_hash)]));
    let dao = MySqlPlayerDao::new(
        executor,
        PlayerPasswordQueries::new(hasher),
        100,
        "Welcome",
        1234,
    );

    assert!(dao.login("alice", "secret").unwrap().is_none());
    assert!(dao.login("alice", "wrong").unwrap().is_none());
}

#[test]
fn updates_player_and_last_login_with_hashed_password_and_timestamp() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let dao = dao(executor);

    dao.update_player(&details()).unwrap();
    dao.update_last_login(&details()).unwrap();

    let plans = dao.executor().plans();
    assert_eq!(plans[0].kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plans[0].sql(),
        "UPDATE users SET password = ?, figure = ?, credits = ?, mission = ?, pool_figure = ?, sex = ?, email = ?, personal_greeting = ?, tickets = ? WHERE id = ?"
    );
    assert!(
        matches!(&plans[0].parameters()[0], SqlParameter::Text(hash) if hash.starts_with("$2a$04$"))
    );
    assert_eq!(
        plans[1].sql(),
        "UPDATE users SET last_online = ? WHERE id = ?"
    );
    assert_eq!(plans[1].parameters()[0], SqlParameter::Long(1234));
}

#[test]
fn validates_executor_result_kind() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let dao = dao(executor);

    assert_eq!(
        dao.details_by_username("alice").unwrap_err().message(),
        "SQL execution kind ReadRows returned affected rows result"
    );
}

#[test]
fn rejects_invalid_login_row_columns() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([SqlRow::new([(
        "id",
        SqlValue::Integer(7),
    )])]));
    let dao = dao(executor);

    assert_eq!(
        dao.login("alice", "secret").unwrap_err().message(),
        "Missing or invalid SQL column `username` as String"
    );
}
