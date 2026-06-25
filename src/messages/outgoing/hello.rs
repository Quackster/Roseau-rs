use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Hello;

impl OutgoingMessage for Hello {
    fn write(&self, response: &mut NettyResponse) {
        response.init("HELLO");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_hello_packet() {
        let mut response = Hello.compose();

        assert_eq!(response.get(), "#HELLO##");
    }
}
