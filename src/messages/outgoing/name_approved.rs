use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NameApproved;

impl OutgoingMessage for NameApproved {
    fn write(&self, response: &mut NettyResponse) {
        response.init("NAME_APPROVED");
    }
}
