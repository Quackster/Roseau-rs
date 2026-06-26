use super::*;

#[test]
fn composes_messenger_message_packet() {
    let mut response =
        MessengerMessage::new(9, 14, "2026-06-24 12:00", "hello", "hr-100").compose();

    assert_eq!(
        response.get(),
        "#MESSENGER_MSG\r9\r14\r[]\r2026-06-24 12:00\rhello\rhr-100\r##"
    );
}
