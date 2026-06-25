use super::player_login_network_plan::*;
use crate::game::player::PlayerDetails;

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        7,
        "alice",
        "mission",
        "figure",
        "pool",
        "alice@example.test",
        1,
        10,
        "F",
        "UK",
        "",
        "1990-01-01",
        1234,
        "hello",
        2,
    );
    details
}

#[test]
fn maps_failed_login_to_current_connection_error() {
    let effects = PlayerLoginNetworkPlan::plan(&PlayerLoginOutcome::failed(), 42);

    assert_eq!(
        effects,
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id: 42,
            packet: "#ERROR Login incorrect##".to_owned(),
        }]
    );
}

#[test]
fn authenticated_login_has_no_direct_error_packet() {
    let outcome =
        PlayerLoginOutcome::authenticated(&details(), "secret", false, 30001, 30001, None);

    assert!(PlayerLoginNetworkPlan::plan(&outcome, 42).is_empty());
}
