use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerInit;

impl IncomingEvent for MessengerInit {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        if context.is_authenticated() && context.is_main_server_connection() {
            context.record(IncomingCommand::InitMessenger);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_messenger_initialisation_when_authenticated() {
        let mut context = IncomingContext::new().authenticated(true);
        MessengerInit.handle(&mut context, &NettyRequest::from_content("MESSENGER_INIT"));

        assert_eq!(context.commands(), &[IncomingCommand::InitMessenger]);
    }

    #[test]
    fn ignores_messenger_initialisation_when_unauthenticated() {
        let mut context = IncomingContext::new();
        MessengerInit.handle(&mut context, &NettyRequest::from_content("MESSENGER_INIT"));

        assert!(context.commands().is_empty());
    }

    #[test]
    fn ignores_messenger_initialisation_off_the_main_server_connection() {
        let mut context = IncomingContext::new()
            .authenticated(true)
            .main_server_connection(false);
        MessengerInit.handle(&mut context, &NettyRequest::from_content("MESSENGER_INIT"));

        assert!(context.commands().is_empty());
    }
}
