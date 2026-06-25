use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerMarkRead;

impl IncomingEvent for MessengerMarkRead {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        if let Ok(message_id) = request.get_message_body().parse::<i32>() {
            context.record(IncomingCommand::MarkMessengerMessageRead { message_id });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_mark_read_command() {
        let mut context = IncomingContext::new();
        MessengerMarkRead.handle(
            &mut context,
            &NettyRequest::from_content("MESSENGER_MARKREAD 77"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::MarkMessengerMessageRead { message_id: 77 }]
        );
    }
}
