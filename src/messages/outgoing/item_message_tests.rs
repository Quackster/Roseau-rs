use super::item_message::*;

#[test]
fn composes_item_message_packet() {
    let mut response = ItemMessage.compose();

    assert_eq!(response.get(), "#ITEMMSG 0\rSELECTTYPE x##");
}
