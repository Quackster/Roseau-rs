use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GiveTickets;

impl IncomingEvent for GiveTickets {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.record(IncomingCommand::SendTickets);
    }
}

#[cfg(test)]
#[path = "give_tickets_tests.rs"]
mod tests;
