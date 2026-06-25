use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct YouAreController;

impl OutgoingMessage for YouAreController {
    fn write(&self, response: &mut NettyResponse) {
        response.init("YOUARECONTROLLER");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_you_are_controller_packet() {
        let mut response = YouAreController.compose();

        assert_eq!(response.get(), "#YOUARECONTROLLER##");
    }
}
