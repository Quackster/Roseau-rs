use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InitUnitListener;

impl IncomingEvent for InitUnitListener {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.record(IncomingCommand::InitUnitListener);
    }
}

#[cfg(test)]
#[path = "init_unit_listener_tests.rs"]
mod tests;
