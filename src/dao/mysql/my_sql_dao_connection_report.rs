use crate::dao::mysql::{MySqlDaoEffect, StorageConnectionOutcome};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MySqlDaoConnectionReport {
    outcome: StorageConnectionOutcome,
    effects: Vec<MySqlDaoEffect>,
}

impl MySqlDaoConnectionReport {
    pub fn new(outcome: StorageConnectionOutcome, effects: impl Into<Vec<MySqlDaoEffect>>) -> Self {
        Self {
            outcome,
            effects: effects.into(),
        }
    }

    pub fn outcome(&self) -> &StorageConnectionOutcome {
        &self.outcome
    }

    pub fn effects(&self) -> &[MySqlDaoEffect] {
        &self.effects
    }

    pub fn connected(&self) -> bool {
        self.outcome.is_connected()
    }

    pub fn error(&self) -> Option<&str> {
        self.outcome.error()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_connection_outcome_and_effects() {
        let report = MySqlDaoConnectionReport::new(
            StorageConnectionOutcome::Failed {
                message: "database unavailable".to_owned(),
            },
            [
                MySqlDaoEffect::ConnectStorage,
                MySqlDaoEffect::LogLine("Could not connect".to_owned()),
            ],
        );

        assert!(!report.connected());
        assert_eq!(report.error(), Some("database unavailable"));
        assert_eq!(
            report.effects(),
            &[
                MySqlDaoEffect::ConnectStorage,
                MySqlDaoEffect::LogLine("Could not connect".to_owned()),
            ]
        );
    }
}
