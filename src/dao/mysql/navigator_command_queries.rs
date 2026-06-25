use crate::dao::mysql::{NavigatorQueries, SqlExecutionPlan};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigatorCommandQueries;

impl NavigatorCommandQueries {
    pub fn plan(effect: &IncomingExecutionEffect) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::SearchFlat { query } => {
                Some(NavigatorQueries::rooms_by_like_name(query).read_plan())
            }
            _ => None,
        }
    }
}
