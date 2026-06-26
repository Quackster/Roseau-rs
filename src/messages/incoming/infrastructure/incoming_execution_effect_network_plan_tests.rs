use super::*;
use crate::game::commands::CommandEffect;

#[test]
fn plans_command_alerts_for_current_connection() {
    let effects = IncomingExecutionEffectNetworkPlan::plan(
        &IncomingExecutionEffect::Command(CommandEffect::SendAlert("hello".to_owned())),
        42,
    );

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#SYSTEMBROADCAST\rhello##".to_owned(),
        }]
    );
}

#[test]
fn leaves_non_network_incoming_effects_to_domain_executors() {
    let effects = IncomingExecutionEffectNetworkPlan::plan_all(
        &[
            IncomingExecutionEffect::Talk {
                mode: "CHAT".to_owned(),
                message: "hello".to_owned(),
            },
            IncomingExecutionEffect::ResetAfkTimer,
            IncomingExecutionEffect::Command(CommandEffect::MarkRoomNeedsUpdate),
        ],
        42,
    );

    assert!(effects.is_empty());
}
