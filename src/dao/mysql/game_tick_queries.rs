use crate::dao::mysql::{PlayerQueries, SqlExecutionPlan};
use crate::game::GameTickEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameTickQueries;

impl GameTickQueries {
    pub fn plan(effect: &GameTickEffect) -> Option<SqlExecutionPlan> {
        match effect {
            GameTickEffect::AwardCredits {
                user_id,
                new_balance,
                ..
            } => Some(Self::update_credits_plan(*user_id, *new_balance)),
            GameTickEffect::SavePlayer { .. }
            | GameTickEffect::ResolveServerIp
            | GameTickEffect::KickAfkUser { .. } => None,
        }
    }

    pub fn update_credits_plan(user_id: i32, new_balance: i32) -> SqlExecutionPlan {
        PlayerQueries::update_credits(user_id, new_balance).execute_plan()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionKind, SqlParameter};

    #[test]
    fn maps_credit_award_effect_to_update_plan() {
        let plan = GameTickQueries::plan(&GameTickEffect::AwardCredits {
            user_id: 7,
            amount: 10,
            new_balance: 125,
        })
        .unwrap();

        assert_eq!(plan.kind(), SqlExecutionKind::Execute);
        assert_eq!(plan.sql(), "UPDATE users SET credits = ? WHERE id = ?");
        assert_eq!(
            plan.parameters(),
            &[SqlParameter::Integer(125), SqlParameter::Integer(7)]
        );
    }

    #[test]
    fn ignores_non_sql_tick_effects() {
        assert_eq!(
            GameTickQueries::plan(&GameTickEffect::SavePlayer { user_id: 7 }),
            None
        );
        assert_eq!(
            GameTickQueries::plan(&GameTickEffect::ResolveServerIp),
            None
        );
        assert_eq!(
            GameTickQueries::plan(&GameTickEffect::KickAfkUser { user_id: 7 }),
            None
        );
    }
}
