use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaoError {
    message: String,
}

impl DaoError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for DaoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for DaoError {}
