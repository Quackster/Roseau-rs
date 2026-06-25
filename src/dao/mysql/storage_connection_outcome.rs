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
mod tests {
    use super::*;

    #[test]
    fn maps_connector_results_to_connection_state() {
        let connected = StorageConnectionOutcome::from_result(Ok(()));
        let failed =
            StorageConnectionOutcome::from_result(Err(DaoError::new("connection refused")));

        assert!(connected.is_connected());
        assert_eq!(connected.error(), None);
        assert!(!failed.is_connected());
        assert_eq!(failed.error(), Some("connection refused"));
    }
}
