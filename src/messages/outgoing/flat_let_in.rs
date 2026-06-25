use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FlatLetIn;

impl OutgoingMessage for FlatLetIn {
    fn write(&self, response: &mut NettyResponse) {
        response.init("FLAT_LETIN");
    }
}
