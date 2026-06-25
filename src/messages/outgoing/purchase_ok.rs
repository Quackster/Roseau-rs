use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PurchaseOk;

impl OutgoingMessage for PurchaseOk {
    fn write(&self, response: &mut NettyResponse) {
        response.init("PURCHASE_OK");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_purchase_ok_packet() {
        let mut response = PurchaseOk.compose();

        assert_eq!(response.get(), "#PURCHASE_OK##");
    }
}
