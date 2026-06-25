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
