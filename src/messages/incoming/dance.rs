use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Dance;

impl IncomingEvent for Dance {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        if !context.is_in_room() {
            return;
        }

        context.record(IncomingCommand::SetRoomStatus {
            key: "dance".to_owned(),
            value: String::new(),
            visible: true,
            timeout: -1,
        });
        context.record(IncomingCommand::MarkRoomNeedsUpdate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_dance_status_when_in_room() {
        let mut context = IncomingContext::new().in_room(true);
        Dance.handle(&mut context, &NettyRequest::from_content("Dance"));

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::SetRoomStatus {
                    key: "dance".to_owned(),
                    value: String::new(),
                    visible: true,
                    timeout: -1,
                },
                IncomingCommand::MarkRoomNeedsUpdate,
            ]
        );
    }
}
