use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EncryptionOff;

impl OutgoingMessage for EncryptionOff {
    fn write(&self, response: &mut NettyResponse) {
        response.init("ENCRYPTION_OFF");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_encryption_off_packet() {
        let mut response = EncryptionOff.compose();

        assert_eq!(response.get(), "#ENCRYPTION_OFF##");
    }
}
