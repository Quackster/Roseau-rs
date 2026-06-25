use crate::dao::mysql::{PlayerQueries, SqlExecutionPlan};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerCommandQueries;

impl PlayerCommandQueries {
    pub fn plan(effect: &IncomingExecutionEffect, user_id: i32) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::AssignPersonalMessage { message } => {
                Some(PlayerQueries::update_personal_greeting(user_id, message).execute_plan())
            }
            IncomingExecutionEffect::UpdatePoolFigure { pool_figure } => {
                Some(PlayerQueries::update_pool_figure(user_id, pool_figure).execute_plan())
            }
            _ => None,
        }
    }

    pub fn read_plan(effect: &IncomingExecutionEffect) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::FindUser { username } => {
                Some(PlayerQueries::details_by_username(username).read_plan())
            }
            _ => None,
        }
    }
}
