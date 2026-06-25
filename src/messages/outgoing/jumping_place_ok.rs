use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JumpingPlaceOk;

impl OutgoingMessage for JumpingPlaceOk {
    fn write(&self, response: &mut NettyResponse) {
        response.init("JUMPINGPLACE_OK");
    }
}

#[cfg(test)]
#[path = "jumping_place_ok_tests.rs"]
mod tests;
