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
mod tests {
    use super::*;

    #[test]
    fn composes_messengers_ready_packet() {
        let mut response = MessengersReady.compose();

        assert_eq!(response.get(), "#MESSENGERSREADY##");
    }
}
