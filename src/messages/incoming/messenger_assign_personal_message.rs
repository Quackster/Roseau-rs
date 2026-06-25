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
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn filters_and_truncates_personal_message() {
        let mut context = IncomingContext::new();
        MessengerAssignPersonalMessage.handle(
            &mut context,
            &NettyRequest::from_content("MESSENGER_ASSIGNPERSMSG abcdefghijklmnopqrstuvwxyz"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::AssignPersonalMessage {
                message: "abcdefghijklmnopqrstu".to_owned(),
            }]
        );
    }
}
