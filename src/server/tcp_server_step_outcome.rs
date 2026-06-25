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
mod tests {
    use super::*;

    #[test]
    fn exposes_common_connection_id_and_read_bytes() {
        let read = TcpServerStepOutcome::Read {
            connection_id: 7,
            bytes_read: 12,
        };
        let idle = TcpServerStepOutcome::Idle { connection_id: 6 };
        let closed = TcpServerStepOutcome::Closed { connection_id: 8 };

        assert_eq!(idle.connection_id(), 6);
        assert_eq!(idle.bytes_read(), None);
        assert_eq!(read.connection_id(), 7);
        assert_eq!(read.bytes_read(), Some(12));
        assert_eq!(closed.connection_id(), 8);
        assert_eq!(closed.bytes_read(), None);
    }
}
