use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct YouAreController;

impl OutgoingMessage for YouAreController {
    fn write(&self, response: &mut NettyResponse) {
        response.init("YOUARECONTROLLER");
    }
}
