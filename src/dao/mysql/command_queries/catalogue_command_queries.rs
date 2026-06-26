use crate::dao::mysql::{CatalogueQueries, SqlExecutionPlan};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogueCommandQueries;

impl CatalogueCommandQueries {
    pub fn read_plans(effect: &IncomingExecutionEffect) -> Vec<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::GetOrderInfo { .. }
            | IncomingExecutionEffect::Purchase { .. } => vec![
                CatalogueQueries::buyable_items().read_plan(),
                CatalogueQueries::item_deals().read_plan(),
            ],
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
#[path = "catalogue_command_queries_tests.rs"]
mod tests;
