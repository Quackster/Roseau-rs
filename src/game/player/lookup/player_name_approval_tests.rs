use super::player_name_approval::*;
use crate::messages::OutgoingMessage;

const ALLOWED: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

#[test]
fn approves_allowed_names_and_packet() {
    let approval = PlayerNameApproval::evaluate("alice1", ALLOWED);
    let mut response = approval.name_approved().unwrap().compose();

    assert_eq!(approval, PlayerNameApproval::Approved);
    assert!(approval.is_approved());
    assert_eq!(response.get(), "#NAME_APPROVED##");
    assert!(approval.name_unacceptable().is_none());
}

#[test]
fn rejects_reserved_prefix_length_and_characters() {
    for name in [
        "ab",
        "thisnameisfartoolonghere",
        "MOD-alice",
        "M0D-alice",
        "ali!",
    ] {
        let approval = PlayerNameApproval::evaluate(name, ALLOWED);

        assert_eq!(approval, PlayerNameApproval::Unacceptable);
        assert!(!approval.is_approved());
    }
}

#[test]
fn wildcard_allows_any_character_after_java_prefix_and_length_checks() {
    assert!(PlayerNameApproval::evaluate("ali!", "*").is_approved());
    assert!(!PlayerNameApproval::evaluate("MOD-alice", "*").is_approved());
}

#[test]
fn maps_unacceptable_name_to_packet() {
    let approval = PlayerNameApproval::evaluate("bad!", ALLOWED);
    let mut response = approval.name_unacceptable().unwrap().compose();

    assert_eq!(response.get(), "#NAME_UNACCEPTABLE##");
    assert!(approval.name_approved().is_none());
}
