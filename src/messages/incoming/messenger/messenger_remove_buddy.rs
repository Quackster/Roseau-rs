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
#[path = "messenger_remove_buddy_tests.rs"]
mod tests;
