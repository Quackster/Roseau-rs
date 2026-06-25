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
mod tests {
    use super::*;

    #[test]
    fn composes_jumping_place_ok_packet() {
        let mut response = JumpingPlaceOk.compose();

        assert_eq!(response.get(), "#JUMPINGPLACE_OK##");
    }
}
