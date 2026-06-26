use std::cell::RefCell;
use std::collections::VecDeque;

use super::*;
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

fn definition(id: i32, behaviour: &str) -> ItemDefinition {
    ItemDefinition::new(id, "chair", "red", 1, 1, 1.0, behaviour, "Chair", "", "")
}

fn definitions() -> HashMap<i32, ItemDefinition> {
    [(5, definition(5, ""))].into_iter().collect()
}

fn definition_row(id: i32, behaviour: &str) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(id)),
        ("sprite", SqlValue::Text("chair".to_owned())),
        ("color", SqlValue::Text("red".to_owned())),
        ("length", SqlValue::Integer(1)),
        ("width", SqlValue::Integer(1)),
        ("height", SqlValue::Float(1.0)),
        ("dataclass", SqlValue::Text(String::new())),
        ("behaviour", SqlValue::Text(behaviour.to_owned())),
        ("name", SqlValue::Text("Chair".to_owned())),
        ("description", SqlValue::Text(String::new())),
    ])
}

fn item_row(id: i32, room_id: i32, owner_id: i32, definition_id: i32) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(id)),
        ("user_id", SqlValue::Integer(owner_id)),
        ("item_id", SqlValue::Integer(definition_id)),
        ("room_id", SqlValue::Integer(room_id)),
        ("x", SqlValue::Text("1".to_owned())),
        ("y", SqlValue::Integer(2)),
        ("z", SqlValue::Float(0.5)),
        ("rotation", SqlValue::Integer(4)),
        ("extra_data", SqlValue::Text("ON".to_owned())),
    ])
}

fn public_item_row(id: i32, definition_id: i32) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(id)),
        ("model", SqlValue::Text("pool_a".to_owned())),
        ("x", SqlValue::Text("4".to_owned())),
        ("y", SqlValue::Integer(5)),
        ("z", SqlValue::Float(1.0)),
        ("rotation", SqlValue::Integer(2)),
        ("definitionid", SqlValue::Integer(definition_id)),
        ("object", SqlValue::Text("chair".to_owned())),
        ("data", SqlValue::Text("ON".to_owned())),
    ])
}

#[test]
fn loads_definitions_through_read_plan() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([definition_row(5, "SFC")]));
    let dao = MySqlItemDao::new(executor, HashMap::new());

    let definitions = dao.definitions().unwrap();

    assert_eq!(definitions[&5].sprite(), "chair");
    assert!(definitions[&5].behaviour().can_sit_on_top());
    let plans = dao.executor().plans();
    assert_eq!(plans[0].kind(), SqlExecutionKind::ReadRows);
    assert_eq!(plans[0].sql(), "SELECT * FROM item_definitions");
}

#[test]
fn loads_room_public_and_single_items_using_cached_definitions() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([item_row(10, 42, 7, 5)]));
    executor.push_result(SqlExecutionResult::rows([public_item_row(20, 5)]));
    executor.push_result(SqlExecutionResult::rows([item_row(11, 42, 7, 5)]));
    let dao = MySqlItemDao::new(executor, definitions());

    let room_items = dao.room_items(42).unwrap();
    let public_items = dao.public_room_items("pool_a", 77).unwrap();
    let item = dao.item(11).unwrap().unwrap();

    assert_eq!(room_items[&10].position().x(), 1);
    assert_eq!(public_items[&20].room_id(), 77);
    assert_eq!(item.id(), 11);
    let plans = dao.executor().plans();
    assert_eq!(plans[0].sql(), "SELECT * FROM items WHERE room_id = ?");
    assert_eq!(
        plans[1].sql(),
        "SELECT * FROM room_public_items WHERE model = ?"
    );
    assert_eq!(plans[2].sql(), "SELECT * FROM items WHERE id = ? LIMIT 1");
}

#[test]
fn saves_and_deletes_items_as_mutations() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let dao = MySqlItemDao::new(executor, definitions());
    let item = Item::new(
        10,
        42,
        7,
        "1",
        2,
        0.5,
        4,
        definition(5, ""),
        "",
        Some("ON".to_owned()),
    )
    .unwrap();

    dao.save_item(&item).unwrap();
    dao.delete_item(10).unwrap();

    let plans = dao.executor().plans();
    assert_eq!(plans[0].kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plans[0].sql(),
        "UPDATE items SET extra_data = ?, x = ?, y = ?, z = ?, rotation = ?, room_id = ?, user_id = ? WHERE id = ?"
    );
    assert_eq!(
        plans[0].parameters()[0],
        SqlParameter::Text("ON".to_owned())
    );
    assert_eq!(plans[1].sql(), "DELETE FROM items WHERE id = ?");
    assert_eq!(plans[1].parameters(), &[SqlParameter::Long(10)]);
}

#[test]
fn validates_result_kinds_and_missing_definitions() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let dao = MySqlItemDao::new(executor, definitions());
    assert_eq!(
        dao.definitions().unwrap_err().message(),
        "SQL execution kind ReadRows returned affected rows result"
    );

    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([item_row(10, 42, 7, 99)]));
    let dao = MySqlItemDao::new(executor, definitions());
    assert_eq!(
        dao.room_items(42).unwrap_err().message(),
        "Missing item definition 99"
    );
}
