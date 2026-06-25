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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::commands::CommandEffect;

    #[test]
    fn collects_direct_incoming_network_effects() {
        let plans = RoseauIncomingExecutionRuntimePlan::collect(
            &[IncomingExecutionEffect::Command(CommandEffect::SendAlert(
                "hello".to_owned(),
            ))],
            42,
        );

        assert_eq!(
            plans,
            vec![RoseauIncomingExecutionRuntimePlan::Network(
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 42,
                    packet: "#SYSTEMBROADCAST\rhello##".to_owned(),
                }
            )]
        );
    }

    #[test]
    fn leaves_domain_only_effects_out_of_runtime_network_plans() {
        let plans = RoseauIncomingExecutionRuntimePlan::collect(
            &[
                IncomingExecutionEffect::ResetAfkTimer,
                IncomingExecutionEffect::Talk {
                    mode: "CHAT".to_owned(),
                    message: "hello".to_owned(),
                },
            ],
            42,
        );

        assert!(plans.is_empty());
    }
}
