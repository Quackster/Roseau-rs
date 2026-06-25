use super::sql_execution_batch_result::*;
use crate::dao::mysql::{SqlParameter, SqlQuery};

fn insert_plan() -> SqlExecutionPlan {
    SqlExecutionPlan::insert_returning_id(SqlQuery::new(
        "INSERT INTO items (item_id) VALUES (?)",
        [SqlParameter::Integer(7)],
    ))
}

fn execute_plan() -> SqlExecutionPlan {
    SqlExecutionPlan::execute(SqlQuery::new("UPDATE users SET credits = 0", []))
}

#[test]
fn validates_batch_results_against_plans_in_order() {
    let plans = [insert_plan(), execute_plan()];
    let result = SqlExecutionBatchResult::new([
        SqlExecutionResult::insert_id(10),
        SqlExecutionResult::affected_rows(1),
    ]);

    assert_eq!(
        result.validate_for(&plans).unwrap(),
        vec![
            SqlExecutionResult::InsertId(10),
            SqlExecutionResult::AffectedRows(1)
        ]
    );
}

#[test]
fn reports_batch_length_mismatch() {
    let result = SqlExecutionBatchResult::new([SqlExecutionResult::insert_id(10)]);

    assert_eq!(
        result
            .validate_for(&[insert_plan(), execute_plan()])
            .unwrap_err()
            .message(),
        "SQL execution batch returned 1 results for 2 plans"
    );
}

#[test]
fn reports_mismatched_result_position() {
    let result = SqlExecutionBatchResult::new([
        SqlExecutionResult::affected_rows(1),
        SqlExecutionResult::affected_rows(1),
    ]);

    assert_eq!(
        result
            .validate_for(&[insert_plan(), execute_plan()])
            .unwrap_err()
            .message(),
        "SQL execution batch result 0 failed validation: SQL execution kind InsertReturningId returned affected rows result"
    );
}

#[test]
fn extracts_insert_ids_after_validation() {
    let plans = [insert_plan(), insert_plan(), execute_plan()];
    let result = SqlExecutionBatchResult::new([
        SqlExecutionResult::insert_id(10),
        SqlExecutionResult::insert_id(11),
        SqlExecutionResult::affected_rows(1),
    ]);

    assert_eq!(result.insert_ids_for(&plans).unwrap(), vec![10, 11]);
}

#[test]
fn extracts_i32_insert_ids_after_validation() {
    let plans = [insert_plan(), insert_plan()];
    let result = SqlExecutionBatchResult::new([
        SqlExecutionResult::insert_id(10),
        SqlExecutionResult::insert_id(11),
    ]);

    assert_eq!(
        result.i32_insert_ids_for(&plans, "item id").unwrap(),
        vec![10, 11]
    );
}

#[test]
fn rejects_i32_insert_id_overflow() {
    let result =
        SqlExecutionBatchResult::new([SqlExecutionResult::insert_id(i64::from(i32::MAX) + 1)]);

    assert_eq!(
        result
            .i32_insert_ids_for(&[insert_plan()], "item id")
            .unwrap_err()
            .message(),
        "Generated item id 2147483648 exceeds i32"
    );
}
