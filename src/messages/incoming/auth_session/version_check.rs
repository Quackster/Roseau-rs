use crate::messages::outgoing::{EncryptionOff, SecretKey};
use crate::messages::{IncomingContext, IncomingEvent, OutgoingMessage};
use crate::protocol::ClientMessage;

/// Matches the original Java server: the version check always answers with
/// ENCRYPTION_OFF followed by the fixed V1 secret key, and the session stays
/// unencrypted.
const V1_SECRET_KEY: &str = "31vw2swky25q9ko940i8x068ftxrmt0wa3vgj27qtrr3m35rn067o549fl";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VersionCheck;

impl IncomingEvent for VersionCheck {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.send(EncryptionOff.compose());
        context.send(SecretKey::new(V1_SECRET_KEY).compose());
    }
}

#[cfg(test)]
#[path = "version_check_tests.rs"]
mod tests;
