use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BadName;

impl OutgoingMessage for BadName {
    fn write(&self, response: &mut NettyResponse) {
        response.init("BADNAME");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_bad_name_packet() {
        let mut response = BadName.compose();

        assert_eq!(response.get(), "#BADNAME##");
    }
}
