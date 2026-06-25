use super::roseau_incoming_execution_runtime_plan::*;
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
