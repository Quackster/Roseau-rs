use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct YouAreNotController;

impl OutgoingMessage for YouAreNotController {
    fn write(&self, response: &mut NettyResponse) {
        response.init("YOUARENOTCONTROLLER");
    }
}

#[cfg(test)]
#[path = "you_are_not_controller_tests.rs"]
mod tests;
