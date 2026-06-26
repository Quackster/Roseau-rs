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
#[path = "my_sql_dao_connection_report_tests.rs"]
mod tests;
