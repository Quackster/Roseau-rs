use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CloseUimakoppi;

impl IncomingEvent for CloseUimakoppi {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        if !context.is_in_room() {
            return;
        }

        context.record(IncomingCommand::ClosePoolChangeBooth);
    }
}

#[cfg(test)]
#[path = "close_uimakoppi_tests.rs"]
mod tests;
