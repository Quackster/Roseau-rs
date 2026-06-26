use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CarryDrink;

impl IncomingEvent for CarryDrink {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let item = request.get_message_body().replace('/', "?");

        context.record(IncomingCommand::RemoveRoomStatus {
            key: "dance".to_owned(),
        });
        context.record(IncomingCommand::SetRoomStatus {
            key: "carryd".to_owned(),
            value: format!(" {item}"),
            visible: false,
            timeout: context.carry_drink_time_value(),
        });
        context.record(IncomingCommand::MarkRoomNeedsUpdate);
    }
}

#[cfg(test)]
#[path = "carry_drink_tests.rs"]
mod tests;
