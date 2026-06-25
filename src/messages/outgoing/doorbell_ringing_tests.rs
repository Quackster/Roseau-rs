use super::doorbell_ringing::*;

#[test]
fn composes_doorbell_ringing_packet() {
    let mut response = DoorbellRinging::new("alice").compose();

    assert_eq!(response.get(), "#DOORBELL_RINGING\ralice##");
}
