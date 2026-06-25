use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BadName;

impl OutgoingMessage for BadName {
    fn write(&self, response: &mut NettyResponse) {
        response.init("BADNAME");
    }
}

#[cfg(test)]
#[path = "bad_name_tests.rs"]
mod tests;
