use super::room_entry_network_plan::*;
use crate::game::room::RoomEffect;

#[test]
fn maps_let_in_to_current_connection_packet() {
    let effects = RoomEntryNetworkPlan::plan(&RoomEntryOutcome::LetIn, 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#FLAT_LETIN##".to_owned(),
        }]
    );
}

#[test]
fn maps_rejected_entry_to_current_connection_error() {
    let effects = RoomEntryNetworkPlan::plan(&RoomEntryOutcome::IncorrectPassword, 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#ERROR Incorrect flat password##".to_owned(),
        }]
    );
}

#[test]
fn leaves_doorbell_routing_to_room_effect_network_planning() {
    let effects = RoomEntryNetworkPlan::plan(
        &RoomEntryOutcome::Doorbell(vec![RoomEffect::SendDoorbell {
            user_id: 7,
            username: "alice".to_owned(),
        }]),
        42,
    );

    assert!(effects.is_empty());
}
