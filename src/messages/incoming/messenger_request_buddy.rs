use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerRequestBuddy;

impl IncomingEvent for MessengerRequestBuddy {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(username) = request.get_argument_with(0, "\r") else {
            return;
        };

        if !username.is_empty() {
            context.record(IncomingCommand::RequestBuddy {
                username: username.to_owned(),
            });
        }
    }
}
