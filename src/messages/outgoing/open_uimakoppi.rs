use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OpenUimakoppi;

impl OutgoingMessage for OpenUimakoppi {
    fn write(&self, response: &mut NettyResponse) {
        response.init("OPEN_UIMAKOPPI");
    }
}

#[cfg(test)]
#[path = "open_uimakoppi_tests.rs"]
mod tests;
