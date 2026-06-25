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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn removes_dance_status_when_requested() {
        let mut context = IncomingContext::new().in_room(true);
        Stop.handle(&mut context, &NettyRequest::from_content("STOP Dance"));

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::RemoveRoomStatus {
                    key: "dance".to_owned(),
                },
                IncomingCommand::MarkRoomNeedsUpdate,
            ]
        );
    }
}
