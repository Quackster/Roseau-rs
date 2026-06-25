use super::my_persistent_message::*;

#[test]
fn composes_my_persistent_message_packet() {
    let mut response = MyPersistentMessage::new("at work").compose();

    assert_eq!(response.get(), "#MYPERSISTENTMSG\rat work##");
}
