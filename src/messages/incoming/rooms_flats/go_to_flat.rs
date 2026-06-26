use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GoToFlat;

impl IncomingEvent for GoToFlat {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.record(IncomingCommand::GoToFlat);
    }
}

#[cfg(test)]
#[path = "go_to_flat_tests.rs"]
mod tests;
