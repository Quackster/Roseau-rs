#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TcpServerAcceptOutcome {
    Skipped,
    Idle,
    Accepted { connection_id: i32 },
    Error { message: String },
}

impl TcpServerAcceptOutcome {
    pub fn accepted_connection_id(&self) -> Option<i32> {
        match self {
            Self::Accepted { connection_id } => Some(*connection_id),
            Self::Skipped | Self::Idle | Self::Error { .. } => None,
        }
    }

    pub fn error(&self) -> Option<&str> {
        match self {
            Self::Error { message } => Some(message),
            Self::Skipped | Self::Idle | Self::Accepted { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_accepted_connection_and_error_message() {
        let accepted = TcpServerAcceptOutcome::Accepted { connection_id: 9 };
        let error = TcpServerAcceptOutcome::Error {
            message: "listener missing".to_owned(),
        };

        assert_eq!(accepted.accepted_connection_id(), Some(9));
        assert_eq!(accepted.error(), None);
        assert_eq!(error.accepted_connection_id(), None);
        assert_eq!(error.error(), Some("listener missing"));
    }
}
