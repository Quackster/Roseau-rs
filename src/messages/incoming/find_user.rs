use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FindUser;

impl IncomingEvent for FindUser {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(username) = request.get_argument_with(0, "\t") else {
            return;
        };

        if !username.is_empty() {
            context.record(IncomingCommand::FindUser {
                username: username.to_owned(),
            });
        }
    }
}

#[cfg(test)]
#[path = "find_user_tests.rs"]
mod tests;
