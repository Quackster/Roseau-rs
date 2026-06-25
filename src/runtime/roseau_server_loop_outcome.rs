use crate::runtime::RoseauStartupRuntimeError;
use crate::server::TcpServerTickOutcome;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoseauServerLoopOutcome {
    Continue { tick: TcpServerTickOutcome },
    Stop { error: RoseauStartupRuntimeError },
}

impl RoseauServerLoopOutcome {
    pub fn from_tick_result(
        result: Result<TcpServerTickOutcome, RoseauStartupRuntimeError>,
    ) -> Self {
        match result {
            Ok(tick) => Self::Continue { tick },
            Err(error) => Self::Stop { error },
        }
    }

    pub fn should_continue(&self) -> bool {
        matches!(self, Self::Continue { .. })
    }

    pub fn tick(&self) -> Option<&TcpServerTickOutcome> {
        match self {
            Self::Continue { tick } => Some(tick),
            Self::Stop { .. } => None,
        }
    }

    pub fn error(&self) -> Option<&RoseauStartupRuntimeError> {
        match self {
            Self::Continue { .. } => None,
            Self::Stop { error } => Some(error),
        }
    }
}
