use std::collections::BTreeMap;

use crate::dao::{mysql::SqlValue, DaoError};

#[derive(Debug, Clone, PartialEq)]
pub struct SqlRow {
    values: BTreeMap<String, SqlValue>,
}

impl SqlRow {
    pub fn new(values: impl IntoIterator<Item = (impl Into<String>, SqlValue)>) -> Self {
        Self {
            values: values
                .into_iter()
                .map(|(name, value)| (name.into(), value))
                .collect(),
        }
    }

    pub fn values(&self) -> &BTreeMap<String, SqlValue> {
        &self.values
    }

    pub fn get(&self, column: &str) -> Option<&SqlValue> {
        self.values.get(column)
    }

    pub fn required_i32(&self, column: &'static str) -> Result<i32, DaoError> {
        self.get(column)
            .and_then(SqlValue::as_i32)
            .ok_or_else(|| missing_column(column, "i32"))
    }

    pub fn required_i64(&self, column: &'static str) -> Result<i64, DaoError> {
        self.get(column)
            .and_then(SqlValue::as_i64)
            .ok_or_else(|| missing_column(column, "i64"))
    }

    pub fn required_f64(&self, column: &'static str) -> Result<f64, DaoError> {
        self.get(column)
            .and_then(SqlValue::as_f64)
            .ok_or_else(|| missing_column(column, "f64"))
    }

    pub fn required_string(&self, column: &'static str) -> Result<String, DaoError> {
        self.get(column)
            .and_then(SqlValue::as_str)
            .map(ToOwned::to_owned)
            .ok_or_else(|| missing_column(column, "String"))
    }

    pub fn required_bool(&self, column: &'static str) -> Result<bool, DaoError> {
        self.get(column)
            .and_then(SqlValue::as_bool)
            .ok_or_else(|| missing_column(column, "bool"))
    }

    pub fn optional_i32(&self, column: &'static str) -> Result<Option<i32>, DaoError> {
        self.optional_column(column, "i32", SqlValue::as_i32)
    }

    pub fn optional_i64(&self, column: &'static str) -> Result<Option<i64>, DaoError> {
        self.optional_column(column, "i64", SqlValue::as_i64)
    }

    pub fn optional_f64(&self, column: &'static str) -> Result<Option<f64>, DaoError> {
        self.optional_column(column, "f64", SqlValue::as_f64)
    }

    pub fn optional_string(&self, column: &'static str) -> Result<Option<String>, DaoError> {
        self.optional_column(column, "String", |value| {
            value.as_str().map(ToOwned::to_owned)
        })
    }

    pub fn optional_bool(&self, column: &'static str) -> Result<Option<bool>, DaoError> {
        self.optional_column(column, "bool", SqlValue::as_bool)
    }

    fn optional_column<T>(
        &self,
        column: &'static str,
        expected_type: &'static str,
        read: impl FnOnce(&SqlValue) -> Option<T>,
    ) -> Result<Option<T>, DaoError> {
        match self.get(column) {
            Some(value) if value.is_null() => Ok(None),
            Some(value) => read(value)
                .map(Some)
                .ok_or_else(|| missing_column(column, expected_type)),
            None => Err(missing_column(column, expected_type)),
        }
    }
}

fn missing_column(column: &'static str, expected_type: &'static str) -> DaoError {
    DaoError::new(format!(
        "Missing or invalid SQL column `{column}` as {expected_type}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_required_typed_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(7)),
            ("name", SqlValue::Text("alice".to_owned())),
            ("last_online", SqlValue::Long(1234)),
            ("height", SqlValue::Float(1.25)),
            ("enabled", SqlValue::Bool(true)),
            ("hidden", SqlValue::Integer(0)),
        ]);

        assert_eq!(row.required_i32("id").unwrap(), 7);
        assert_eq!(row.required_i64("last_online").unwrap(), 1234);
        assert_eq!(row.required_f64("height").unwrap(), 1.25);
        assert_eq!(row.required_string("name").unwrap(), "alice");
        assert!(row.required_bool("enabled").unwrap());
        assert!(!row.required_bool("hidden").unwrap());
    }

    #[test]
    fn reports_missing_or_invalid_columns() {
        let row = SqlRow::new([("id", SqlValue::Text("not a number".to_owned()))]);

        assert_eq!(
            row.required_i32("id").unwrap_err().message(),
            "Missing or invalid SQL column `id` as i32"
        );
        assert_eq!(
            row.required_string("name").unwrap_err().message(),
            "Missing or invalid SQL column `name` as String"
        );
        assert_eq!(
            row.required_bool("name").unwrap_err().message(),
            "Missing or invalid SQL column `name` as bool"
        );
    }

    #[test]
    fn reads_nullable_typed_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Null),
            ("owner_id", SqlValue::Integer(7)),
            ("height", SqlValue::Float(1.5)),
            ("name", SqlValue::Text("alice".to_owned())),
            ("enabled", SqlValue::Long(1)),
            ("invalid", SqlValue::Text("not a number".to_owned())),
        ]);

        assert_eq!(row.optional_i32("id").unwrap(), None);
        assert_eq!(row.optional_i32("owner_id").unwrap(), Some(7));
        assert_eq!(row.optional_i64("owner_id").unwrap(), Some(7));
        assert_eq!(row.optional_f64("height").unwrap(), Some(1.5));
        assert_eq!(
            row.optional_string("name").unwrap(),
            Some("alice".to_owned())
        );
        assert_eq!(row.optional_bool("enabled").unwrap(), Some(true));
        assert_eq!(
            row.optional_i32("invalid").unwrap_err().message(),
            "Missing or invalid SQL column `invalid` as i32"
        );
        assert_eq!(
            row.optional_string("missing").unwrap_err().message(),
            "Missing or invalid SQL column `missing` as String"
        );
    }
}
