use crate::messages::outgoing::{EncryptionOn, SecretKey};
use crate::messages::{IncomingContext, IncomingEvent, OutgoingMessage};
use crate::protocol::ClientMessage;

pub const V1_SECRET_KEY: &str = "ABAB";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VersionCheck;

impl IncomingEvent for VersionCheck {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.send(EncryptionOn.compose());
        context.send(SecretKey::new(V1_SECRET_KEY).compose());
    }
}

#[cfg(test)]
#[path = "version_check_tests.rs"]
mod tests;
