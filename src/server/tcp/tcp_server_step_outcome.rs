#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TcpServerStepOutcome {
    Idle {
        connection_id: i32,
    },
    Read {
        connection_id: i32,
        bytes_read: usize,
    },
    Closed {
        connection_id: i32,
    },
    Error {
        connection_id: i32,
        message: String,
    },
}

impl TcpServerStepOutcome {
    pub fn connection_id(&self) -> i32 {
        match self {
            Self::Idle { connection_id }
            | Self::Read { connection_id, .. }
            | Self::Closed { connection_id }
            | Self::Error { connection_id, .. } => *connection_id,
        }
    }

    pub fn bytes_read(&self) -> Option<usize> {
        match self {
            Self::Read { bytes_read, .. } => Some(*bytes_read),
            Self::Idle { .. } | Self::Closed { .. } | Self::Error { .. } => None,
        }
    }
}

#[cfg(test)]
#[path = "tcp_server_step_outcome_tests.rs"]
mod tests;
