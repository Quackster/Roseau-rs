use super::*;

#[test]
fn created_registration_emits_no_direct_packet_like_java() {
    let effects = PlayerRegistrationNetworkPlan::plan(PlayerRegistrationOutcome::Created, 42);

    assert!(effects.is_empty());
}

#[test]
fn taken_name_emits_no_direct_packet_like_java() {
    let effects = PlayerRegistrationNetworkPlan::plan(PlayerRegistrationOutcome::NameTaken, 42);

    assert!(effects.is_empty());
}
