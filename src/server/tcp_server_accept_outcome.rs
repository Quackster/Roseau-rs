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
