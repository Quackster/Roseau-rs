use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NameUnacceptable;

impl OutgoingMessage for NameUnacceptable {
    fn write(&self, response: &mut NettyResponse) {
        response.init("NAME_UNACCEPTABLE");
    }
}

#[cfg(test)]
#[path = "name_unacceptable_tests.rs"]
mod tests;
