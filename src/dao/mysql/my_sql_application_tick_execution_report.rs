use crate::dao::mysql::SqlExecutionBatchResult;
use crate::game::{GameTickEffect, GameTickRuntimeEffect};

#[derive(Debug, Clone, PartialEq)]
pub struct MySqlApplicationTickExecutionReport {
    database_result: SqlExecutionBatchResult,
    runtime_effects: Vec<GameTickEffect>,
}

impl MySqlApplicationTickExecutionReport {
    pub fn new(
        database_result: SqlExecutionBatchResult,
        runtime_effects: impl Into<Vec<GameTickEffect>>,
    ) -> Self {
        Self {
            database_result,
            runtime_effects: runtime_effects.into(),
        }
    }

    pub fn database_result(&self) -> &SqlExecutionBatchResult {
        &self.database_result
    }

    pub fn runtime_effects(&self) -> &[GameTickEffect] {
        &self.runtime_effects
    }

    pub fn runtime_actions(&self) -> Vec<GameTickRuntimeEffect> {
        GameTickRuntimeEffect::collect(&self.runtime_effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_database_result_and_runtime_effects() {
        let report = MySqlApplicationTickExecutionReport::new(
            SqlExecutionBatchResult::new([]),
            [
                GameTickEffect::AwardCredits {
                    user_id: 3,
                    amount: 10,
                    new_balance: 20,
                },
                GameTickEffect::ResolveServerIp,
                GameTickEffect::KickAfkUser { user_id: 7 },
            ],
        );

        assert!(report.database_result().results().is_empty());
        assert_eq!(
            report.runtime_effects(),
            &[
                GameTickEffect::AwardCredits {
                    user_id: 3,
                    amount: 10,
                    new_balance: 20,
                },
                GameTickEffect::ResolveServerIp,
                GameTickEffect::KickAfkUser { user_id: 7 },
            ]
        );
        assert_eq!(
            report.runtime_actions(),
            vec![
                GameTickRuntimeEffect::SendCreditBalance {
                    user_id: 3,
                    new_balance: 20,
                },
                GameTickRuntimeEffect::ResolveServerIp,
                GameTickRuntimeEffect::KickAfkUser { user_id: 7 },
            ]
        );
    }
}
