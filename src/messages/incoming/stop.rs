use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Stop;

impl IncomingEvent for Stop {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        if !context.is_in_room() || request.get_argument_amount() < 1 {
            return;
        }

        if request.get_argument(0) == Some("Dance") {
            context.record(IncomingCommand::RemoveRoomStatus {
                key: "dance".to_owned(),
            });
            context.record(IncomingCommand::MarkRoomNeedsUpdate);
        }
    }
}
