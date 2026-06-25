use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KillUser;

impl IncomingEvent for KillUser {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let username = request.get_message_body();
        context.record(IncomingCommand::ResetAfkTimer);

        if !username.is_empty() {
            context.record(IncomingCommand::KickUser { username });
        }
    }
}
