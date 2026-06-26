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
#[path = "purchase_add_strip_item_tests.rs"]
mod tests;
