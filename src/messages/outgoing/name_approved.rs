use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NameApproved;

impl OutgoingMessage for NameApproved {
    fn write(&self, response: &mut NettyResponse) {
        response.init("NAME_APPROVED");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_name_approved_packet() {
        let mut response = NameApproved.compose();

        assert_eq!(response.get(), "#NAME_APPROVED##");
    }
}
