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
#[path = "messenger_decline_buddy_tests.rs"]
mod tests;
