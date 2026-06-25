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
mod tests {
    use super::*;
    use crate::messages::outgoing::{StripInfo, StripItem, StripItemKind};

    #[test]
    fn maps_refreshed_inventory_to_current_connection_packet() {
        let execution = InventoryCommandExecution::Refreshed {
            strip_info: StripInfo::new([StripItem::new(
                7,
                "chair",
                "Chair",
                "blue",
                StripItemKind::Stuff {
                    length: 1,
                    width: 2,
                    color: "red".to_owned(),
                },
            )]),
        };

        let effects = InventoryCommandNetworkPlan::plan(&execution, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#STRIPINFO\rroseau;7;0;S;0;chair;Chair;blue;1;2;red/##".to_owned(),
            }]
        );
    }

    #[test]
    fn empty_inventory_refresh_has_no_network_effect() {
        assert!(
            InventoryCommandNetworkPlan::plan(&InventoryCommandExecution::Empty, 42).is_empty()
        );
    }
}
