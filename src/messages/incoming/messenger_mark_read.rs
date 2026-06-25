use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerMarkRead;

impl IncomingEvent for MessengerMarkRead {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        if let Ok(message_id) = request.get_message_body().parse::<i32>() {
            context.record(IncomingCommand::MarkMessengerMessageRead { message_id });
        }
    }
}
