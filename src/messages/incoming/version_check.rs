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
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn sends_encryption_off_and_secret_key() {
        let mut context = IncomingContext::new();
        VersionCheck.handle(&mut context, &NettyRequest::from_content("VERSIONCHECK"));

        let mut first = context.sent()[0].clone();
        let mut second = context.sent()[1].clone();

        assert_eq!(first.get(), "#ENCRYPTION_OFF##");
        assert_eq!(
            second.get(),
            "#SECRET_KEY\r31vw2swky25q9ko940i8x068ftxrmt0wa3vgj27qtrr3m35rn067o549fl##"
        );
    }
}
