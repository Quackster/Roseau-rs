use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerAcceptBuddy;

impl IncomingEvent for MessengerAcceptBuddy {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let username = request.get_message_body();
        if !username.is_empty() {
            context.record(IncomingCommand::AcceptBuddy { username });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_accept_buddy_command() {
        let mut context = IncomingContext::new();
        MessengerAcceptBuddy.handle(
            &mut context,
            &NettyRequest::from_content("MESSENGER_ACCEPTBUDDY alice"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::AcceptBuddy {
                username: "alice".to_owned(),
            }]
        );
    }
}
