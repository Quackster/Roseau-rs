use super::*;

#[test]
fn maps_approved_name_to_current_connection_packet() {
    let effects = PlayerNameApprovalNetworkPlan::plan(PlayerNameApproval::Approved, 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#NAME_APPROVED##".to_owned(),
        }]
    );
}

#[test]
fn maps_unacceptable_name_to_current_connection_packet() {
    let effects = PlayerNameApprovalNetworkPlan::plan(PlayerNameApproval::Unacceptable, 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#NAME_UNACCEPTABLE##".to_owned(),
        }]
    );
}
