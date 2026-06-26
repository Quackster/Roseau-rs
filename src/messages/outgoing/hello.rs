use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Hello;

impl OutgoingMessage for Hello {
    fn write(&self, response: &mut NettyResponse) {
        response.init("HELLO");
    }
}

#[cfg(test)]
#[path = "hello_tests.rs"]
mod tests;
