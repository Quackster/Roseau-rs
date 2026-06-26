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
#[path = "messenger_ready_tests.rs"]
mod tests;
