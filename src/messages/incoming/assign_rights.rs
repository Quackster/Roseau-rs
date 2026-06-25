use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AssignRights;

impl IncomingEvent for AssignRights {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let username = request.get_message_body();
        if !username.is_empty() {
            context.record(IncomingCommand::AssignRights { username });
        }
    }
}

#[cfg(test)]
#[path = "assign_rights_tests.rs"]
mod tests;
