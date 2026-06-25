use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CarryItem;

impl IncomingEvent for CarryItem {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let item = request.get_message_body().replace('/', "?");

        context.record(IncomingCommand::RemoveRoomStatus {
            key: "dance".to_owned(),
        });
        context.record(IncomingCommand::SetRoomStatus {
            key: "carryd".to_owned(),
            value: format!(" {item}"),
            visible: false,
            timeout: 0,
        });
        context.record(IncomingCommand::MarkRoomNeedsUpdate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_carry_status() {
        let mut context = IncomingContext::new();
        CarryItem.handle(
            &mut context,
            &NettyRequest::from_content("CarryItem cola/bottle"),
        );

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::RemoveRoomStatus {
                    key: "dance".to_owned(),
                },
                IncomingCommand::SetRoomStatus {
                    key: "carryd".to_owned(),
                    value: " cola?bottle".to_owned(),
                    visible: false,
                    timeout: 0,
                },
                IncomingCommand::MarkRoomNeedsUpdate,
            ]
        );
    }
}
