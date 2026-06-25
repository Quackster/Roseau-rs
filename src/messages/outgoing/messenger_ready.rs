use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerReady;

impl OutgoingMessage for MessengerReady {
    fn write(&self, response: &mut NettyResponse) {
        response.init("MESSENGERREADY");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_messenger_ready_packet() {
        let mut response = MessengerReady.compose();

        assert_eq!(response.get(), "#MESSENGERREADY##");
    }
}
