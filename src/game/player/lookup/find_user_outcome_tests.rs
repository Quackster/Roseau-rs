use super::find_user_outcome::*;
use crate::messages::OutgoingMessage;

fn details() -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        7,
        "alice",
        "hello",
        "hd-100",
        "",
        "alice@example.test",
        1,
        50,
        "F",
        "UK",
        "",
        "1990-01-01",
        1234,
        "welcome",
        3,
    );
    details
}

#[test]
fn maps_found_player_to_member_info_packet() {
    let outcome = FindUserOutcome::found(&details(), "now", "On Hotel View");

    assert_eq!(
        outcome.member_info().unwrap().compose().get(),
        "#MEMBERINFO \ralice\rwelcome\rnow\rOn Hotel View\rhd-100##"
    );
    assert!(outcome.no_such_user().is_none());
}

#[test]
fn maps_missing_player_to_no_such_user_packet() {
    let outcome = FindUserOutcome::Missing;

    assert!(outcome.member_info().is_none());
    assert_eq!(
        outcome.no_such_user().unwrap().compose().get(),
        "#NOSUCHUSER##"
    );
}
