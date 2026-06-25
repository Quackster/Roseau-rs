use crate::game::catalogue::CatalogueOrderInfoPlan;
use crate::messages::outgoing::OrderInfo;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CatalogueOrderInfoNetworkPlan;

impl CatalogueOrderInfoNetworkPlan {
    pub fn plan(plan: &CatalogueOrderInfoPlan, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        let mut response = OrderInfo::new(plan.call_id(), plan.credits()).compose();
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id,
            packet: response.get(),
        }]
    }

    pub fn plan_all(
        plans: &[CatalogueOrderInfoPlan],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        plans
            .iter()
            .flat_map(|plan| Self::plan(plan, connection_id))
            .collect()
    }
}

#[cfg(test)]
#[path = "catalogue_order_info_network_plan_tests.rs"]
mod tests;
