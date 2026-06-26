use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoseauStartupRuntimeError {
    NotListening,
}

impl Display for RoseauStartupRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotListening => write!(f, "startup runtime is not listening"),
        }
    }
}

impl std::error::Error for RoseauStartupRuntimeError {}

#[cfg(test)]
#[path = "roseau_startup_runtime_error_tests.rs"]
mod tests;
