use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;
use crate::util::filter_input;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerAssignPersonalMessage;

impl IncomingEvent for MessengerAssignPersonalMessage {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let mut message = filter_input(&request.get_message_body());

        if message.chars().count() > 23 {
            message = message.chars().take(21).collect();
        }

        context.record(IncomingCommand::AssignPersonalMessage { message });
    }
}

#[cfg(test)]
#[path = "messenger_assign_personal_message_tests.rs"]
mod tests;
