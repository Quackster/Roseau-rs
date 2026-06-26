use super::*;

#[test]
fn stores_messenger_message_fields() {
    let message = MessengerMessage::new(1, 2, 3, 12345, "hello");

    assert_eq!(message.id(), 1);
    assert_eq!(message.to_id(), 2);
    assert_eq!(message.from_id(), 3);
    assert_eq!(message.time_sent(), 12345);
    assert_eq!(message.message(), "hello");
}
