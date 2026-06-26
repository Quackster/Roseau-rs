use crate::dao::DaoError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageConnectionOutcome {
    Connected,
    Failed { message: String },
}

impl StorageConnectionOutcome {
    pub fn from_result(result: Result<(), DaoError>) -> Self {
        match result {
            Ok(()) => Self::Connected,
            Err(error) => Self::Failed {
                message: error.message().to_owned(),
            },
        }
    }

    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected)
    }

    pub fn error(&self) -> Option<&str> {
        match self {
            Self::Connected => None,
            Self::Failed { message } => Some(message),
        }
    }
}

#[cfg(test)]
#[path = "storage_connection_outcome_tests.rs"]
mod tests;
