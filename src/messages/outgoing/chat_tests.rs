use super::chat::*;

#[test]
fn composes_chat_packet_with_dynamic_header() {
    let mut response = Chat::new("CHAT", "alice", "hello#there").compose();

    assert_eq!(response.get(), "#CHAT\ralice hello*there##");
}
