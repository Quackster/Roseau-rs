use crate::messages::outgoing::{EncryptionOff, SecretKey};
use crate::messages::{IncomingContext, IncomingEvent, OutgoingMessage};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VersionCheck;

impl IncomingEvent for VersionCheck {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.send(EncryptionOff.compose());
        context.send(
            SecretKey::new("31vw2swky25q9ko940i8x068ftxrmt0wa3vgj27qtrr3m35rn067o549fl").compose(),
        );
    }
}

#[cfg(test)]
#[path = "version_check_tests.rs"]
mod tests;
