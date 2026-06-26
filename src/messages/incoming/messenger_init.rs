use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerInit;

impl IncomingEvent for MessengerInit {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        if context.is_main_server_connection() {
            context.record(IncomingCommand::InitMessenger);
        }
    }
}

#[cfg(test)]
#[path = "messenger_init_tests.rs"]
mod tests;
