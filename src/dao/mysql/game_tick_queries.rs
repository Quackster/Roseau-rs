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
#[path = "game_tick_queries_tests.rs"]
mod tests;
