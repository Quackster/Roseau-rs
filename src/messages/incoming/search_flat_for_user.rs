use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SearchFlatForUser;

impl IncomingEvent for SearchFlatForUser {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(username) = request.get_argument_with(1, "/") else {
            return;
        };

        if !username.is_empty() {
            context.record(IncomingCommand::SearchFlatForUser {
                username: username.to_owned(),
            });
        }
    }
}

#[cfg(test)]
#[path = "search_flat_for_user_tests.rs"]
mod tests;
