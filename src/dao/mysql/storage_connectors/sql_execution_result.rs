use crate::dao::mysql::{SqlExecutionKind, SqlRow};
use crate::dao::DaoError;

#[derive(Debug, Clone, PartialEq)]
pub enum SqlExecutionResult {
    Rows(Vec<SqlRow>),
    AffectedRows(u64),
    InsertId(i64),
}

impl SqlExecutionResult {
    pub fn rows(rows: impl IntoIterator<Item = SqlRow>) -> Self {
        Self::Rows(rows.into_iter().collect())
    }

    pub fn affected_rows(count: u64) -> Self {
        Self::AffectedRows(count)
    }

    pub fn insert_id(id: i64) -> Self {
        Self::InsertId(id)
    }

    pub fn require_rows(self) -> Result<Vec<SqlRow>, DaoError> {
        match self {
            Self::Rows(rows) => Ok(rows),
            result => Err(result_kind_error(result.kind_name(), "read rows")),
        }
    }

    pub fn optional_first_row(self) -> Result<Option<SqlRow>, DaoError> {
        Ok(self.require_rows()?.into_iter().next())
    }

    pub fn has_rows(self) -> Result<bool, DaoError> {
        Ok(!self.require_rows()?.is_empty())
    }

    pub fn map_rows<T>(
        self,
        mut map_row: impl FnMut(&SqlRow) -> Result<T, DaoError>,
    ) -> Result<Vec<T>, DaoError> {
        self.require_rows()?
            .iter()
            .map(|row| map_row(row))
            .collect()
    }

    pub fn require_affected_rows(self) -> Result<u64, DaoError> {
        match self {
            Self::AffectedRows(count) => Ok(count),
            result => Err(result_kind_error(result.kind_name(), "affected rows")),
        }
    }

    pub fn require_mutation(self) -> Result<(), DaoError> {
        self.require_affected_rows().map(|_| ())
    }

    pub fn require_insert_id(self) -> Result<i64, DaoError> {
        match self {
            Self::InsertId(id) => Ok(id),
            result => Err(result_kind_error(result.kind_name(), "insert id")),
        }
    }

    pub fn require_i32_insert_id(self, generated_id_label: &'static str) -> Result<i32, DaoError> {
        let id = self.require_insert_id()?;
        i32::try_from(id)
            .map_err(|_| DaoError::new(format!("Generated {generated_id_label} {id} exceeds i32")))
    }

    pub fn validate_for(self, kind: SqlExecutionKind) -> Result<Self, DaoError> {
        let valid = matches!(
            (kind, &self),
            (SqlExecutionKind::ReadRows, Self::Rows(_))
                | (SqlExecutionKind::Execute, Self::AffectedRows(_))
                | (SqlExecutionKind::InsertReturningId, Self::InsertId(_))
        );

        if valid {
            Ok(self)
        } else {
            Err(DaoError::new(format!(
                "SQL execution kind {:?} returned {} result",
                kind,
                self.kind_name()
            )))
        }
    }

    fn kind_name(&self) -> &'static str {
        match self {
            Self::Rows(_) => "rows",
            Self::AffectedRows(_) => "affected rows",
            Self::InsertId(_) => "insert id",
        }
    }
}

fn result_kind_error(actual: &'static str, expected: &'static str) -> DaoError {
    DaoError::new(format!(
        "SQL execution result contains {actual}, expected {expected}"
    ))
}

#[cfg(test)]
#[path = "sql_execution_result_tests.rs"]
mod tests;
