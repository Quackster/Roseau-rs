use super::room_decoration_network_plan::*;

#[test]
fn maps_applied_decoration_to_current_connection_packet() {
    let effects =
        RoomDecorationNetworkPlan::plan(&RoomDecorationOutcome::applied("floor", "wood"), 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#FLATPROPERTY\rfloor/wood##".to_owned(),
        }]
    );
}

#[test]
fn ignored_decoration_has_no_network_effect() {
    assert!(RoomDecorationNetworkPlan::plan(&RoomDecorationOutcome::Ignored, 42).is_empty());
}
