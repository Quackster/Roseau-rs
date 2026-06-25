use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GetFlatInfo;

impl IncomingEvent for GetFlatInfo {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(room_id) = request.get_argument_with(1, "/") else {
            return;
        };

        let Ok(room_id) = room_id.parse::<i32>() else {
            return;
        };

        context.record(IncomingCommand::GetFlatInfo { room_id });
    }
}
