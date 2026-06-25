use super::player_login_outcome::*;
use crate::messages::OutgoingMessage;

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
fn maps_successful_main_server_login_to_authenticated_details_and_last_login() {
    let outcome =
        PlayerLoginOutcome::authenticated(&details(), "secret", false, 30001, 30001, None);

    let authenticated = outcome.details().unwrap();
    assert!(authenticated.is_authenticated());
    assert_eq!(authenticated.password(), "secret");
    assert_eq!(
        outcome.effects(),
        &[PlayerEffect::UpdateLastLogin { user_id: 7 }]
    );
    assert_eq!(outcome.public_room_lookup_id(), None);
    assert!(outcome.login_error().is_none());
}

#[test]
fn maps_room_login_to_duplicate_close_and_public_room_lookup() {
    let outcome =
        PlayerLoginOutcome::authenticated(&details(), "secret", true, 30045, 30001, Some(11));

    assert_eq!(
        outcome.effects(),
        &[
            PlayerEffect::CloseConnection { connection_id: 11 },
            PlayerEffect::UpdateLastLogin { user_id: 7 },
        ]
    );
    assert_eq!(outcome.public_room_lookup_id(), Some(44));
}

#[test]
fn maps_failed_login_to_java_error_packet() {
    let outcome = PlayerLoginOutcome::failed();
    let mut response = outcome.login_error().unwrap().compose();

    assert_eq!(response.get(), "#ERROR Login incorrect##");
    assert!(outcome.details().is_none());
    assert!(outcome.effects().is_empty());
}
