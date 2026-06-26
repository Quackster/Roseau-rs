use super::*;
use crate::messages::OutgoingMessage;

#[test]
fn maps_let_in_to_flat_let_in_packet() {
    let mut response = RoomEntryOutcome::LetIn.flat_let_in().unwrap().compose();

    assert_eq!(response.get(), "#FLAT_LETIN##");
    assert!(RoomEntryOutcome::LetIn.error().is_none());
}

#[test]
fn maps_rejected_entry_to_java_error_packet() {
    let mut response = RoomEntryOutcome::IncorrectPassword
        .error()
        .unwrap()
        .compose();

    assert_eq!(response.get(), "#ERROR Incorrect flat password##");
    assert!(RoomEntryOutcome::IncorrectPassword.flat_let_in().is_none());
}

#[test]
fn exposes_doorbell_effects_without_packets() {
    let outcome = RoomEntryOutcome::Doorbell(vec![RoomEffect::SendDoorbell {
        user_id: 7,
        username: "alice".to_owned(),
    }]);

    assert_eq!(
        outcome.doorbell_effects(),
        &[RoomEffect::SendDoorbell {
            user_id: 7,
            username: "alice".to_owned(),
        }]
    );
    assert!(outcome.flat_let_in().is_none());
    assert!(outcome.error().is_none());
}
