use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GetCredits;

impl IncomingEvent for GetCredits {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.record(IncomingCommand::GetCredits);
    }
}

#[cfg(test)]
#[path = "get_credits_tests.rs"]
mod tests;
