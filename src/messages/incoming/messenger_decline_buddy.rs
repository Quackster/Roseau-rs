use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerDeclineBuddy;

impl IncomingEvent for MessengerDeclineBuddy {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let username = request.get_message_body();
        if !username.is_empty() {
            context.record(IncomingCommand::DeclineBuddy { username });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_decline_buddy_command() {
        let mut context = IncomingContext::new();
        MessengerDeclineBuddy.handle(
            &mut context,
            &NettyRequest::from_content("MESSENGER_DECLINEBUDDY alice"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::DeclineBuddy {
                username: "alice".to_owned(),
            }]
        );
    }
}
