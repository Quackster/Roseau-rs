use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FlatLetIn;

impl OutgoingMessage for FlatLetIn {
    fn write(&self, response: &mut NettyResponse) {
        response.init("FLAT_LETIN");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_flat_let_in_packet() {
        let mut response = FlatLetIn.compose();

        assert_eq!(response.get(), "#FLAT_LETIN##");
    }
}
