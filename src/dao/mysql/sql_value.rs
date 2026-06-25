#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    Null,
    Bool(bool),
    Integer(i32),
    Long(i64),
    Float(f64),
    Text(String),
}

impl SqlValue {
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
}

impl From<bool> for SqlValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for SqlValue {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<i64> for SqlValue {
    fn from(value: i64) -> Self {
        Self::Long(value)
    }
}

impl From<f64> for SqlValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<String> for SqlValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for SqlValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_typed_sql_values() {
        assert_eq!(SqlValue::from(7).as_i32(), Some(7));
        assert_eq!(SqlValue::from(7).as_i64(), Some(7));
        assert_eq!(SqlValue::from(7_i64).as_i64(), Some(7));
        assert_eq!(SqlValue::from(1.5).as_f64(), Some(1.5));
        assert_eq!(SqlValue::from("alice").as_str(), Some("alice"));
        assert_eq!(SqlValue::from(true).as_bool(), Some(true));
        assert_eq!(SqlValue::from(1).as_bool(), Some(true));
        assert_eq!(SqlValue::from(0).as_bool(), Some(false));
        assert!(SqlValue::Null.is_null());
    }
}
