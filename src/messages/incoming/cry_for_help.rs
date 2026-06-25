use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CryForHelp;

impl IncomingEvent for CryForHelp {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        if !context.is_in_room() {
            return;
        }

        let message = request.get_message_body();
        if message.is_empty() {
            return;
        }

        context.record(IncomingCommand::CryForHelp { message });
    }
}
