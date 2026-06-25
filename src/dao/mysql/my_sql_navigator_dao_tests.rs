use std::cell::RefCell;
use std::collections::VecDeque;

use super::my_sql_navigator_dao::*;
use crate::dao::mysql::{SqlExecutionKind, SqlParameter, SqlRow, SqlValue};
use crate::game::room::settings::RoomType;

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

fn room_row(id: i32, name: &str) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(id)),
        ("name", SqlValue::Text(name.to_owned())),
        ("order_id", SqlValue::Integer(2)),
        (
            "room_type",
            SqlValue::Integer(RoomType::Private.type_code()),
        ),
        ("enabled", SqlValue::Integer(1)),
        ("hidden", SqlValue::Integer(0)),
        ("owner_id", SqlValue::Integer(5)),
        ("description", SqlValue::Text("Private room".to_owned())),
        ("password", SqlValue::Text(String::new())),
        ("state", SqlValue::Integer(0)),
        ("show_owner_name", SqlValue::Integer(1)),
        ("allsuperuser", SqlValue::Integer(0)),
        ("users_now", SqlValue::Integer(3)),
        ("users_max", SqlValue::Integer(25)),
        ("cct", SqlValue::Text("hh_room".to_owned())),
        ("model", SqlValue::Text("model_a".to_owned())),
        ("wallpaper", SqlValue::Text("101".to_owned())),
        ("floor", SqlValue::Text("201".to_owned())),
    ])
}

#[test]
fn loads_private_rooms_by_like_name_through_read_plan() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([room_row(10, "Cafe")]));
    let dao = MySqlNavigatorDao::new(executor, "alice");

    let rooms = dao.rooms_by_like_name("caf").unwrap();

    assert_eq!(rooms.len(), 1);
    assert_eq!(rooms[0].id(), 10);
    assert_eq!(rooms[0].name(), "Cafe");
    assert_eq!(rooms[0].owner_name(), "alice");
    let plans = dao.executor().plans();
    assert_eq!(plans[0].kind(), SqlExecutionKind::ReadRows);
    assert_eq!(
        plans[0].sql(),
        "SELECT * FROM rooms WHERE name LIKE ? AND room_type = ?"
    );
    assert_eq!(
        plans[0].parameters(),
        &[
            SqlParameter::Text("%caf%".to_owned()),
            SqlParameter::Integer(RoomType::Private.type_code()),
        ]
    );
}

#[test]
fn validates_executor_result_kind_before_mapping() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let dao = MySqlNavigatorDao::new(executor, "alice");

    assert_eq!(
        dao.rooms_by_like_name("caf").unwrap_err().message(),
        "SQL execution kind ReadRows returned affected rows result"
    );
}
