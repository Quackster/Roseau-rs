use super::messengers_ready::*;

#[test]
fn composes_messengers_ready_packet() {
    let mut response = MessengersReady.compose();

    assert_eq!(response.get(), "#MESSENGERSREADY##");
}
