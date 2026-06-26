use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IntoDoor;

impl IncomingEvent for IntoDoor {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Ok(item_id) = request.get_message_body().parse::<i32>() else {
            return;
        };

        if context.is_in_room() {
            context.record(IncomingCommand::EnterDoor { item_id });
        }
    }
}

#[cfg(test)]
#[path = "into_door_tests.rs"]
mod tests;
