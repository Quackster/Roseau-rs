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
mod tests {
    use super::*;

    #[test]
    fn composes_you_are_not_controller_packet() {
        let mut response = YouAreNotController.compose();

        assert_eq!(response.get(), "#YOUARENOTCONTROLLER##");
    }
}
