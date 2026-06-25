use crate::messages::{IncomingExecutionEffect, IncomingExecutionEffectNetworkPlan};
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoseauIncomingExecutionRuntimePlan {
    Network(PlayerNetworkEffect),
}

impl RoseauIncomingExecutionRuntimePlan {
    pub fn collect(effects: &[IncomingExecutionEffect], connection_id: i32) -> Vec<Self> {
        IncomingExecutionEffectNetworkPlan::plan_all(effects, connection_id)
            .into_iter()
            .map(Self::Network)
            .collect()
    }
}
