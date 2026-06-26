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

fn definition(id: i32) -> ItemDefinition {
    ItemDefinition::new(id, "chair", "red", 1, 1, 1.0, "", "Chair", "", "")
}

fn definitions() -> HashMap<i32, ItemDefinition> {
    [(5, definition(5))].into_iter().collect()
}

fn item_row(id: i32, owner_id: i32, definition_id: i32) -> SqlRow {
    SqlRow::new([
        ("id", SqlValue::Integer(id)),
        ("user_id", SqlValue::Integer(owner_id)),
        ("item_id", SqlValue::Integer(definition_id)),
        ("room_id", SqlValue::Integer(0)),
        ("x", SqlValue::Text("0".to_owned())),
        ("y", SqlValue::Integer(0)),
        ("z", SqlValue::Float(0.0)),
        ("rotation", SqlValue::Integer(0)),
        ("extra_data", SqlValue::Text("ON".to_owned())),
    ])
}

#[test]
fn loads_inventory_items_through_read_plan() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([
        item_row(11, 7, 5),
        item_row(10, 7, 5),
    ]));
    let dao = MySqlInventoryDao::new(executor, definitions());

    let items = dao.inventory_items(7).unwrap();

    assert_eq!(items.iter().map(Item::id).collect::<Vec<_>>(), vec![10, 11]);
    assert_eq!(items[0].definition().id(), 5);
    let plans = dao.executor().plans();
    assert_eq!(plans[0].kind(), SqlExecutionKind::ReadRows);
    assert_eq!(
        plans[0].sql(),
        "SELECT * FROM items WHERE room_id = ? AND user_id = ?"
    );
    assert_eq!(
        plans[0].parameters(),
        &[SqlParameter::Integer(0), SqlParameter::Integer(7)]
    );
}

#[test]
fn loads_optional_inventory_item() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::rows([item_row(10, 7, 5)]));
    let dao = MySqlInventoryDao::new(executor, definitions());

    let item = dao.item(10).unwrap().unwrap();

    assert_eq!(item.id(), 10);
    assert_eq!(item.custom_data(), Some("ON"));
    let plans = dao.executor().plans();
    assert_eq!(plans[0].sql(), "SELECT * FROM items WHERE id = ? LIMIT 1");
    assert_eq!(plans[0].parameters(), &[SqlParameter::Long(10)]);
}

#[test]
fn creates_inventory_item_from_insert_id_and_definition() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::insert_id(77));
    let dao = MySqlInventoryDao::new(executor, definitions());

    let item = dao.new_item(5, 7, "red").unwrap();

    assert_eq!(item.id(), 77);
    assert_eq!(item.room_id(), 0);
    assert_eq!(item.owner_id(), 7);
    assert_eq!(item.custom_data(), Some("red"));
    let plans = dao.executor().plans();
    assert_eq!(plans[0].kind(), SqlExecutionKind::InsertReturningId);
    assert_eq!(
        plans[0].sql(),
        "INSERT INTO items (user_id, item_id, room_id, x, extra_data) VALUES (?, ?, ?, ?, ?)"
    );
}

#[test]
fn rejects_wrong_result_kind_missing_definition_and_large_insert_id() {
    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::affected_rows(1));
    let dao = MySqlInventoryDao::new(executor, definitions());
    assert_eq!(
        dao.inventory_items(7).unwrap_err().message(),
        "SQL execution kind ReadRows returned affected rows result"
    );

    let executor = RecordingExecutor::default();
    let dao = MySqlInventoryDao::new(executor, HashMap::new());
    assert_eq!(
        dao.new_item(5, 7, "red").unwrap_err().message(),
        "Missing item definition 5"
    );

    let executor = RecordingExecutor::default();
    executor.push_result(SqlExecutionResult::insert_id(i64::from(i32::MAX) + 1));
    let dao = MySqlInventoryDao::new(executor, definitions());
    assert_eq!(
        dao.new_item(5, 7, "red").unwrap_err().message(),
        "Generated inventory item id 2147483648 exceeds i32"
    );
}
