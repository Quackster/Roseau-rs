use crate::dao::{DaoError, InventoryDao};
use crate::game::inventory::{InventoryCommandExecution, InventoryCommandExecutor};
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InventoryIncomingPlan;

impl InventoryIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        inventory_dao: &dyn InventoryDao,
        user_id: i32,
    ) -> Result<Vec<InventoryCommandExecution>, DaoError> {
        let IncomingExecutionEffect::RefreshInventory { category } = effect else {
            return Ok(Vec::new());
        };

        Ok(vec![InventoryCommandExecutor::refresh_inventory(
            inventory_dao,
            user_id,
            category,
        )?])
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        inventory_dao: &dyn InventoryDao,
        user_id: i32,
    ) -> Result<Vec<InventoryCommandExecution>, DaoError> {
        let mut executions = Vec::new();

        for effect in effects {
            executions.extend(Self::plan(effect, inventory_dao, user_id)?);
        }

        Ok(executions)
    }
}
