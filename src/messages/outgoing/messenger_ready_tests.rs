use super::messenger_ready::*;

#[test]
fn composes_messenger_ready_packet() {
    let mut response = MessengerReady.compose();

    assert_eq!(response.get(), "#MESSENGERREADY##");
}
