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
mod tests {
    use super::*;

    #[test]
    fn formats_not_listening_error() {
        assert_eq!(
            RoseauStartupRuntimeError::NotListening.to_string(),
            "startup runtime is not listening"
        );
    }
}
