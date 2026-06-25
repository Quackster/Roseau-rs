use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Ok;

impl OutgoingMessage for Ok {
    fn write(&self, response: &mut NettyResponse) {
        response.init("OK");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_ok_packet() {
        let mut response = Ok.compose();

        assert_eq!(response.get(), "#OK##");
    }
}
