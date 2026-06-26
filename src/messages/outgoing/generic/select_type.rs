use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SelectType;

impl OutgoingMessage for SelectType {
    fn write(&self, response: &mut NettyResponse) {
        response.init("SELECTTYPE");
        response.append_new_argument("x");
    }
}

#[cfg(test)]
#[path = "select_type_tests.rs"]
mod tests;
