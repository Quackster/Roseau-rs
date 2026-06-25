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
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_carry_drink_status() {
        let mut context = IncomingContext::new().carry_drink_time(180);
        CarryDrink.handle(&mut context, &NettyRequest::from_content("CarryDrink tea"));

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::RemoveRoomStatus {
                    key: "dance".to_owned(),
                },
                IncomingCommand::SetRoomStatus {
                    key: "carryd".to_owned(),
                    value: " tea".to_owned(),
                    visible: false,
                    timeout: 180,
                },
                IncomingCommand::MarkRoomNeedsUpdate,
            ]
        );
    }
}
