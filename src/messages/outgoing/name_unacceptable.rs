use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NameUnacceptable;

impl OutgoingMessage for NameUnacceptable {
    fn write(&self, response: &mut NettyResponse) {
        response.init("NAME_UNACCEPTABLE");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_name_unacceptable_packet() {
        let mut response = NameUnacceptable.compose();

        assert_eq!(response.get(), "#NAME_UNACCEPTABLE##");
    }
}
