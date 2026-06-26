use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InfoRetrieve;

impl IncomingEvent for InfoRetrieve {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.record(IncomingCommand::RetrieveUserInfo);
    }
}

#[cfg(test)]
#[path = "info_retrieve_tests.rs"]
mod tests;
