use super::*;

#[test]
fn plans_alert_command_as_system_broadcast_to_current_connection() {
    let effects = CommandEffectNetworkPlan::plan(&CommandEffect::SendAlert("hello".to_owned()), 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#SYSTEMBROADCAST\rhello##".to_owned(),
        }]
    );
}

#[test]
fn leaves_non_network_command_effects_for_other_executors() {
    let effects = CommandEffectNetworkPlan::plan_all(
        &[
            CommandEffect::RemoveRoomStatus {
                key: "sit".to_owned(),
            },
            CommandEffect::SetRoomStatus {
                key: "sit".to_owned(),
                value: "1.0".to_owned(),
                infinite: true,
                duration: -1,
            },
            CommandEffect::MarkRoomNeedsUpdate,
            CommandEffect::ReloadItemDefinitions,
        ],
        42,
    );

    assert!(effects.is_empty());
}
