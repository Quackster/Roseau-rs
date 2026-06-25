use crate::game::inventory::InventoryCommandExecution;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InventoryCommandNetworkPlan;

impl InventoryCommandNetworkPlan {
    pub fn plan(
        execution: &InventoryCommandExecution,
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        execution
            .strip_info()
            .map(|strip_info| {
                let mut response = strip_info.compose();
                vec![PlayerNetworkEffect::WriteResponse {
                    connection_id,
                    packet: response.get(),
                }]
            })
            .unwrap_or_default()
    }

    pub fn plan_all(
        executions: &[InventoryCommandExecution],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        executions
            .iter()
            .flat_map(|execution| Self::plan(execution, connection_id))
            .collect()
    }
}

#[cfg(test)]
#[path = "inventory_command_network_plan_tests.rs"]
mod tests;
