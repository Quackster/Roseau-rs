use crate::dao::DaoError;
use crate::game::inventory::{InventoryCommandExecution, InventoryCommandExecutor};
use crate::game::item::Item;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InventoryIncomingPlan;

impl InventoryIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        items: &[Item],
    ) -> Result<Vec<InventoryCommandExecution>, DaoError> {
        let IncomingExecutionEffect::RefreshInventory { category } = effect else {
            return Ok(Vec::new());
        };

        Ok(vec![InventoryCommandExecutor::refresh_inventory(
            items,
            category,
        )?])
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        items: &[Item],
    ) -> Result<Vec<InventoryCommandExecution>, DaoError> {
        let mut executions = Vec::new();

        for effect in effects {
            executions.extend(Self::plan(effect, items)?);
        }

        Ok(executions)
    }
}

#[cfg(test)]
#[path = "inventory_incoming_plan_tests.rs"]
mod tests;
