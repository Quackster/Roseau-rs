use crate::server::{TcpServerAcceptOutcome, TcpServerStepOutcome};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcpServerTickOutcome {
    accept_outcome: TcpServerAcceptOutcome,
    read_outcomes: Vec<TcpServerStepOutcome>,
    removed_connection_ids: Vec<i32>,
}

impl TcpServerTickOutcome {
    pub fn new(
        accept_outcome: TcpServerAcceptOutcome,
        read_outcomes: impl Into<Vec<TcpServerStepOutcome>>,
        removed_connection_ids: impl Into<Vec<i32>>,
    ) -> Self {
        Self {
            accept_outcome,
            read_outcomes: read_outcomes.into(),
            removed_connection_ids: removed_connection_ids.into(),
        }
    }

    pub fn accept_outcome(&self) -> &TcpServerAcceptOutcome {
        &self.accept_outcome
    }

    pub fn accepted_connection_id(&self) -> Option<i32> {
        self.accept_outcome.accepted_connection_id()
    }

    pub fn accept_error(&self) -> Option<&str> {
        self.accept_outcome.error()
    }

    pub fn read_outcomes(&self) -> &[TcpServerStepOutcome] {
        &self.read_outcomes
    }

    pub fn removed_connection_ids(&self) -> &[i32] {
        &self.removed_connection_ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_tick_components() {
        let outcome = TcpServerTickOutcome::new(
            TcpServerAcceptOutcome::Accepted { connection_id: 5 },
            [TcpServerStepOutcome::Closed { connection_id: 4 }],
            [4],
        );

        assert_eq!(
            outcome.accept_outcome(),
            &TcpServerAcceptOutcome::Accepted { connection_id: 5 }
        );
        assert_eq!(outcome.accepted_connection_id(), Some(5));
        assert_eq!(outcome.accept_error(), None);
        assert_eq!(
            outcome.read_outcomes(),
            &[TcpServerStepOutcome::Closed { connection_id: 4 }]
        );
        assert_eq!(outcome.removed_connection_ids(), &[4]);
    }
}
