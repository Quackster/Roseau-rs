use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemMessage;

impl OutgoingMessage for ItemMessage {
    fn write(&self, response: &mut NettyResponse) {
        response.init("ITEMMSG 0");
        response.append_new_argument("SELECTTYPE x");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_item_message_packet() {
        let mut response = ItemMessage.compose();

        assert_eq!(response.get(), "#ITEMMSG 0\rSELECTTYPE x##");
    }
}
