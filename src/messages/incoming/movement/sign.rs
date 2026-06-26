use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Sign;

impl IncomingEvent for Sign {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        if !context.is_in_room_model("pool_b") {
            return;
        }

        let Ok(sign_id) = request.get_message_body().parse::<i32>() else {
            return;
        };

        if !(1..=14).contains(&sign_id) {
            return;
        }

        context.record(IncomingCommand::RemoveRoomStatus {
            key: "dance".to_owned(),
        });
        context.record(IncomingCommand::SetRoomStatus {
            key: "sign".to_owned(),
            value: format!(" {sign_id}"),
            visible: false,
            timeout: 2,
        });
        context.record(IncomingCommand::MarkRoomNeedsUpdate);
    }
}

#[cfg(test)]
#[path = "sign_tests.rs"]
mod tests;
