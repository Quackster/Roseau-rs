use super::*;

#[test]
fn preserves_read_query_sql_and_parameters() {
    let plan = SqlExecutionPlan::read_rows(SqlQuery::new(
        "SELECT * FROM users WHERE id = ?",
        [SqlParameter::Integer(7)],
    ));

    assert_eq!(plan.kind(), SqlExecutionKind::ReadRows);
    assert_eq!(plan.sql(), "SELECT * FROM users WHERE id = ?");
    assert_eq!(plan.parameters(), &[SqlParameter::Integer(7)]);
}

#[test]
fn preserves_update_query_as_execute_plan() {
    let plan = SqlExecutionPlan::execute(SqlQuery::new(
        "UPDATE users SET credits = ? WHERE id = ?",
        [SqlParameter::Integer(100), SqlParameter::Integer(7)],
    ));

    assert_eq!(plan.kind(), SqlExecutionKind::Execute);
    assert_eq!(
        plan.parameters(),
        &[SqlParameter::Integer(100), SqlParameter::Integer(7)]
    );
}

#[test]
fn preserves_insert_query_as_returning_id_plan() {
    let plan = SqlExecutionPlan::insert_returning_id(SqlQuery::new(
        "INSERT INTO users (username) VALUES (?)",
        [SqlParameter::Text("alice".to_owned())],
    ));

    assert_eq!(plan.kind(), SqlExecutionKind::InsertReturningId);
    assert_eq!(plan.sql(), "INSERT INTO users (username) VALUES (?)");
}

#[test]
fn validates_driver_results_against_plan_kind() {
    let read_plan = SqlExecutionPlan::read_rows(SqlQuery::new("SELECT * FROM users", []));
    assert!(read_plan
        .validate_result(SqlExecutionResult::rows([]))
        .is_ok());

    let execute_plan = SqlExecutionPlan::execute(SqlQuery::new("DELETE FROM users", []));
    assert_eq!(
        execute_plan
            .validate_result(SqlExecutionResult::insert_id(1))
            .unwrap_err()
            .message(),
        "SQL execution kind Execute returned insert id result"
    );
}

#[test]
fn validates_parameter_count_against_query_placeholders() {
    let matching = SqlExecutionPlan::execute(SqlQuery::new(
        "UPDATE users SET credits = ? WHERE id = ?",
        [SqlParameter::Integer(100), SqlParameter::Integer(7)],
    ));
    let missing = SqlExecutionPlan::execute(SqlQuery::new(
        "UPDATE users SET credits = ? WHERE id = ?",
        [SqlParameter::Integer(100)],
    ));

    assert!(matching.validate_parameters().is_ok());
    assert_eq!(
        missing.validate_parameters().unwrap_err().message(),
        "SQL parameter count mismatch: `UPDATE users SET credits = ? WHERE id = ?` has 2 placeholders but 1 parameters"
    );
}

#[test]
fn exposes_driver_neutral_parameter_values() {
    let plan = SqlExecutionPlan::execute(SqlQuery::new(
        "UPDATE users SET credits = ?, motto = ? WHERE id = ?",
        [
            SqlParameter::Integer(100),
            SqlParameter::Text("hello".to_owned()),
            SqlParameter::Long(7),
        ],
    ));

    assert_eq!(
        plan.parameter_values(),
        vec![
            SqlValue::Integer(100),
            SqlValue::Text("hello".to_owned()),
            SqlValue::Long(7)
        ]
    );
}

#[test]
fn exposes_redacted_parameter_values_for_diagnostics() {
    let plan = SqlExecutionPlan::execute(SqlQuery::new(
        "UPDATE users SET password = ?, credits = ?, enabled = ? WHERE id = ?",
        [
            SqlParameter::Text("hash".to_owned()),
            SqlParameter::Integer(100),
            SqlParameter::Bool(true),
            SqlParameter::Long(7),
        ],
    ));

    assert_eq!(
        plan.redacted_parameter_values(),
        vec![
            "<text>".to_owned(),
            "100".to_owned(),
            "true".to_owned(),
            "7".to_owned()
        ]
    );
}

#[test]
fn exposes_redacted_plan_diagnostic() {
    let plan = SqlExecutionPlan::execute(SqlQuery::new(
        "UPDATE users SET password = ?, credits = ?, enabled = ? WHERE id = ?",
        [
            SqlParameter::Text("hash".to_owned()),
            SqlParameter::Integer(100),
            SqlParameter::Bool(true),
            SqlParameter::Long(7),
        ],
    ));

    assert_eq!(
        plan.redacted_diagnostic(),
        "Execute `UPDATE users SET password = ?, credits = ?, enabled = ? WHERE id = ?` with parameters [<text>, 100, true, 7]"
    );
}

#[test]
fn ignores_quoted_question_marks_when_validating_parameters() {
    let plan = SqlExecutionPlan::execute(SqlQuery::new(
        r#"UPDATE logs SET text = 'literal ?', note = "escaped \"?\"" WHERE id = ?"#,
        [SqlParameter::Integer(7)],
    ));

    assert!(plan.validate_parameters().is_ok());
}
