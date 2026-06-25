use crate::dao::mysql::{InventoryQueries, SqlExecutionPlan};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InventoryCommandQueries;

impl InventoryCommandQueries {
    pub fn read_plan(effect: &IncomingExecutionEffect, user_id: i32) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::RefreshInventory { .. } => {
                Some(InventoryQueries::inventory_items(user_id).read_plan())
            }
            _ => None,
        }
    }
}
