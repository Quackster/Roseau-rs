use crate::dao::mysql::{SqlExecutionKind, SqlExecutionResult, SqlParameter, SqlQuery, SqlValue};
use crate::dao::DaoError;

#[derive(Debug, Clone, PartialEq)]
pub struct SqlExecutionPlan {
    kind: SqlExecutionKind,
    sql: String,
    parameters: Vec<SqlParameter>,
}

impl SqlExecutionPlan {
    pub fn new(kind: SqlExecutionKind, query: SqlQuery) -> Self {
        let (sql, parameters) = query.into_parts();
        Self {
            kind,
            sql,
            parameters,
        }
    }

    pub fn read_rows(query: SqlQuery) -> Self {
        Self::new(SqlExecutionKind::ReadRows, query)
    }

    pub fn execute(query: SqlQuery) -> Self {
        Self::new(SqlExecutionKind::Execute, query)
    }

    pub fn insert_returning_id(query: SqlQuery) -> Self {
        Self::new(SqlExecutionKind::InsertReturningId, query)
    }

    pub fn kind(&self) -> SqlExecutionKind {
        self.kind
    }

    pub fn sql(&self) -> &str {
        &self.sql
    }

    pub fn parameters(&self) -> &[SqlParameter] {
        &self.parameters
    }

    pub fn parameter_values(&self) -> Vec<SqlValue> {
        self.parameters.iter().map(SqlParameter::to_value).collect()
    }

    pub fn redacted_parameter_values(&self) -> Vec<String> {
        self.parameters
            .iter()
            .map(SqlParameter::redacted_display)
            .collect()
    }

    pub fn redacted_diagnostic(&self) -> String {
        format!(
            "{:?} `{}` with parameters [{}]",
            self.kind,
            self.sql,
            self.redacted_parameter_values().join(", ")
        )
    }

    pub fn validate_result(
        &self,
        result: SqlExecutionResult,
    ) -> Result<SqlExecutionResult, DaoError> {
        result.validate_for(self.kind)
    }

    pub fn validate_parameters(&self) -> Result<(), DaoError> {
        let placeholder_count = count_parameter_placeholders(&self.sql);

        if placeholder_count == self.parameters.len() {
            Ok(())
        } else {
            Err(DaoError::new(format!(
                "SQL parameter count mismatch: `{}` has {placeholder_count} placeholders but {} parameters",
                self.sql,
                self.parameters.len()
            )))
        }
    }
}

fn count_parameter_placeholders(sql: &str) -> usize {
    let mut count = 0;
    let mut chars = sql.chars().peekable();
    let mut quoted = None;

    while let Some(ch) = chars.next() {
        if let Some(quote) = quoted {
            if ch == '\\' {
                chars.next();
            } else if ch == quote {
                if chars.peek() == Some(&quote) {
                    chars.next();
                } else {
                    quoted = None;
                }
            }
        } else if ch == '\'' || ch == '"' {
            quoted = Some(ch);
        } else if ch == '?' {
            count += 1;
        }
    }

    count
}
