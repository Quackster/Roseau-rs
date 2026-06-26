use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengersReady;

impl OutgoingMessage for MessengersReady {
    fn write(&self, response: &mut NettyResponse) {
        response.init("MESSENGERSREADY");
    }
}

#[cfg(test)]
#[path = "messengers_ready_tests.rs"]
mod tests;
