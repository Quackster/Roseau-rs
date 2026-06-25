use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PurchaseAddStripItem;

impl OutgoingMessage for PurchaseAddStripItem {
    fn write(&self, response: &mut NettyResponse) {
        response.init("ADDSTRIPITEM");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_purchase_add_strip_item_packet() {
        let mut response = PurchaseAddStripItem.compose();

        assert_eq!(response.get(), "#ADDSTRIPITEM##");
    }
}
