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
mod tests {
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
}
