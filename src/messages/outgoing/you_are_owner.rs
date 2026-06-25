use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct YouAreOwner;

impl OutgoingMessage for YouAreOwner {
    fn write(&self, response: &mut NettyResponse) {
        response.init("YOUAREOWNER");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_you_are_owner_packet() {
        let mut response = YouAreOwner.compose();

        assert_eq!(response.get(), "#YOUAREOWNER##");
    }
}
