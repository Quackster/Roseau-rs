use super::unconfigured_sql_driver::*;
use crate::dao::mysql::{SqlExecutionPlan, SqlParameter, SqlQuery};

#[test]
fn reports_missing_database_driver_for_sql_execution() {
    let error = UnconfiguredSqlDriver::new()
        .execute_plan(&SqlExecutionPlan::execute(SqlQuery::new(
            "UPDATE users SET password = ?, credits = ? WHERE id = ?",
            [
                SqlParameter::Text("hash".to_owned()),
                SqlParameter::Integer(100),
                SqlParameter::Long(7),
            ],
        )))
        .unwrap_err();

    assert_eq!(
        error.message(),
        "SQL driver is not configured for Execute `UPDATE users SET password = ?, credits = ? WHERE id = ?` with parameters [<text>, 100, 7]"
    );
}
