use crate::dao::mysql::{SqlExecutionPlan, SqlParameter};

#[derive(Debug, Clone, PartialEq)]
pub struct SqlQuery {
    sql: String,
    parameters: Vec<SqlParameter>,
}

impl SqlQuery {
    pub fn new(sql: impl Into<String>, parameters: impl IntoIterator<Item = SqlParameter>) -> Self {
        Self {
            sql: sql.into(),
            parameters: parameters.into_iter().collect(),
        }
    }

    pub fn select_all(table: &str) -> Self {
        Self::new(format!("SELECT * FROM {table}"), [])
    }

    pub fn sql(&self) -> &str {
        &self.sql
    }

    pub fn parameters(&self) -> &[SqlParameter] {
        &self.parameters
    }

    pub fn into_parts(self) -> (String, Vec<SqlParameter>) {
        (self.sql, self.parameters)
    }

    pub fn read_plan(self) -> SqlExecutionPlan {
        SqlExecutionPlan::read_rows(self)
    }

    pub fn execute_plan(self) -> SqlExecutionPlan {
        SqlExecutionPlan::execute(self)
    }

    pub fn insert_returning_id_plan(self) -> SqlExecutionPlan {
        SqlExecutionPlan::insert_returning_id(self)
    }
}
