use super::*;
use crate::dao::mysql::SqlValue;

fn row() -> SqlRow {
    SqlRow::new([("id", SqlValue::Integer(7))])
}

#[test]
fn exposes_typed_success_values() {
    assert_eq!(
        SqlExecutionResult::rows([row()]).require_rows().unwrap()[0]
            .required_i32("id")
            .unwrap(),
        7
    );
    assert_eq!(
        SqlExecutionResult::affected_rows(2)
            .require_affected_rows()
            .unwrap(),
        2
    );
    assert!(SqlExecutionResult::affected_rows(2)
        .require_mutation()
        .is_ok());
    assert_eq!(
        SqlExecutionResult::insert_id(9)
            .require_insert_id()
            .unwrap(),
        9
    );
    assert_eq!(
        SqlExecutionResult::insert_id(9)
            .require_i32_insert_id("player id")
            .unwrap(),
        9
    );
}

#[test]
fn exposes_optional_first_row_for_lookup_results() {
    assert_eq!(
        SqlExecutionResult::rows([row()])
            .optional_first_row()
            .unwrap()
            .unwrap()
            .required_i32("id")
            .unwrap(),
        7
    );
    assert!(SqlExecutionResult::rows([])
        .optional_first_row()
        .unwrap()
        .is_none());
}

#[test]
fn exposes_row_presence_for_exists_queries() {
    assert!(SqlExecutionResult::rows([row()]).has_rows().unwrap());
    assert!(!SqlExecutionResult::rows([]).has_rows().unwrap());
}

#[test]
fn maps_validated_rows() {
    assert_eq!(
        SqlExecutionResult::rows([row()])
            .map_rows(|row| row.required_i32("id"))
            .unwrap(),
        vec![7]
    );
}

#[test]
fn rejects_mismatched_accessors() {
    assert_eq!(
        SqlExecutionResult::insert_id(9)
            .require_rows()
            .unwrap_err()
            .message(),
        "SQL execution result contains insert id, expected read rows"
    );
    assert_eq!(
        SqlExecutionResult::rows([row()])
            .require_insert_id()
            .unwrap_err()
            .message(),
        "SQL execution result contains rows, expected insert id"
    );
    assert_eq!(
        SqlExecutionResult::insert_id(i64::from(i32::MAX) + 1)
            .require_i32_insert_id("player id")
            .unwrap_err()
            .message(),
        "Generated player id 2147483648 exceeds i32"
    );
}

#[test]
fn validates_against_plan_kind() {
    assert!(SqlExecutionResult::rows([row()])
        .validate_for(SqlExecutionKind::ReadRows)
        .is_ok());
    assert!(SqlExecutionResult::affected_rows(1)
        .validate_for(SqlExecutionKind::Execute)
        .is_ok());
    assert!(SqlExecutionResult::insert_id(4)
        .validate_for(SqlExecutionKind::InsertReturningId)
        .is_ok());

    assert_eq!(
        SqlExecutionResult::affected_rows(1)
            .validate_for(SqlExecutionKind::ReadRows)
            .unwrap_err()
            .message(),
        "SQL execution kind ReadRows returned affected rows result"
    );
}
