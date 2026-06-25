use super::player_registration_network_plan::*;

#[test]
fn maps_created_registration_to_ok_packet() {
    let effects = PlayerRegistrationNetworkPlan::plan(PlayerRegistrationOutcome::Created, 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#OK##".to_owned(),
        }]
    );
}

#[test]
fn maps_taken_name_to_bad_name_packet() {
    let effects = PlayerRegistrationNetworkPlan::plan(PlayerRegistrationOutcome::NameTaken, 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#BADNAME##".to_owned(),
        }]
    );
}
