use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerRemoveBuddy;

impl IncomingEvent for MessengerRemoveBuddy {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let username = request.get_message_body();
        if !username.is_empty() {
            context.record(IncomingCommand::RemoveBuddy { username });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_remove_buddy_command() {
        let mut context = IncomingContext::new();
        MessengerRemoveBuddy.handle(
            &mut context,
            &NettyRequest::from_content("MESSENGER_REMOVEBUDDY alice"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::RemoveBuddy {
                username: "alice".to_owned(),
            }]
        );
    }
}
