use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct YouAreOwner;

impl OutgoingMessage for YouAreOwner {
    fn write(&self, response: &mut NettyResponse) {
        response.init("YOUAREOWNER");
    }
}
