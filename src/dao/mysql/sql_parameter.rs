use crate::dao::mysql::SqlValue;

#[derive(Debug, Clone, PartialEq)]
pub enum SqlParameter {
    Null,
    Bool(bool),
    Integer(i32),
    Long(i64),
    Float(f64),
    Text(String),
}

impl SqlParameter {
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Self::Bool(value) => Some(i32::from(*value)),
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Bool(value) => Some(i64::from(*value)),
            Self::Integer(value) => Some(i64::from(*value)),
            Self::Long(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            Self::Integer(value) => Some(*value != 0),
            Self::Long(value) => Some(*value != 0),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn redacted_display(&self) -> String {
        match self {
            Self::Null => "NULL".to_owned(),
            Self::Bool(value) => value.to_string(),
            Self::Integer(value) => value.to_string(),
            Self::Long(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::Text(_) => "<text>".to_owned(),
        }
    }

    pub fn to_value(&self) -> SqlValue {
        match self {
            Self::Null => SqlValue::Null,
            Self::Bool(value) => SqlValue::Bool(*value),
            Self::Integer(value) => SqlValue::Integer(*value),
            Self::Long(value) => SqlValue::Long(*value),
            Self::Float(value) => SqlValue::Float(*value),
            Self::Text(value) => SqlValue::Text(value.clone()),
        }
    }
}

impl From<bool> for SqlParameter {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for SqlParameter {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<i64> for SqlParameter {
    fn from(value: i64) -> Self {
        Self::Long(value)
    }
}

impl From<f64> for SqlParameter {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<String> for SqlParameter {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for SqlParameter {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl<T> From<Option<T>> for SqlParameter
where
    T: Into<SqlParameter>,
{
    fn from(value: Option<T>) -> Self {
        value.map(Into::into).unwrap_or(Self::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_typed_sql_parameters() {
        assert_eq!(SqlParameter::from(7).as_i32(), Some(7));
        assert_eq!(SqlParameter::from(7).as_i64(), Some(7));
        assert_eq!(SqlParameter::from(7_i64).as_i64(), Some(7));
        assert_eq!(SqlParameter::from(1.5).as_f64(), Some(1.5));
        assert_eq!(SqlParameter::from("alice").as_str(), Some("alice"));
        assert_eq!(SqlParameter::from(true).as_bool(), Some(true));
        assert_eq!(SqlParameter::from(1).as_bool(), Some(true));
        assert_eq!(SqlParameter::from(0).as_bool(), Some(false));
        assert!(SqlParameter::Null.is_null());
    }

    #[test]
    fn converts_parameters_to_driver_neutral_sql_values() {
        assert_eq!(SqlParameter::Null.to_value(), SqlValue::Null);
        assert_eq!(SqlParameter::from(true).to_value(), SqlValue::Bool(true));
        assert_eq!(SqlParameter::from(7).to_value(), SqlValue::Integer(7));
        assert_eq!(SqlParameter::from(7_i64).to_value(), SqlValue::Long(7));
        assert_eq!(SqlParameter::from(1.5).to_value(), SqlValue::Float(1.5));
        assert_eq!(
            SqlParameter::from("alice").to_value(),
            SqlValue::Text("alice".to_owned())
        );
    }

    #[test]
    fn redacts_text_parameters_for_diagnostics() {
        assert_eq!(SqlParameter::Null.redacted_display(), "NULL");
        assert_eq!(SqlParameter::from(true).redacted_display(), "true");
        assert_eq!(SqlParameter::from(7).redacted_display(), "7");
        assert_eq!(SqlParameter::from(7_i64).redacted_display(), "7");
        assert_eq!(SqlParameter::from(1.5).redacted_display(), "1.5");
        assert_eq!(SqlParameter::from("secret").redacted_display(), "<text>");
    }

    #[test]
    fn converts_optional_values_to_nullable_parameters() {
        assert_eq!(
            SqlParameter::from(Some("hello")),
            SqlParameter::Text("hello".to_owned())
        );
        assert_eq!(SqlParameter::from(Some(7)), SqlParameter::Integer(7));
        assert_eq!(SqlParameter::from(Option::<i32>::None), SqlParameter::Null);
    }
}
