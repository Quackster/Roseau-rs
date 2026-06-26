use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EncryptionOn;

impl OutgoingMessage for EncryptionOn {
    fn write(&self, response: &mut NettyResponse) {
        response.init("ENCRYPTION_ON");
    }
}

#[cfg(test)]
#[path = "encryption_on_tests.rs"]
mod tests;
