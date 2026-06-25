use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerSendMessage;

impl IncomingEvent for MessengerSendMessage {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let body = request.get_message_body();
        let Some((receivers, message)) = body.split_once('\r') else {
            return;
        };

        let mut receiver_ids = Vec::new();
        for receiver in receivers.split(' ') {
            let Ok(receiver_id) = receiver.parse::<i32>() else {
                return;
            };
            receiver_ids.push(receiver_id);
        }

        context.record(IncomingCommand::SendMessengerMessage {
            receiver_ids,
            message: message.replace('\r', "\n"),
        });
    }
}

#[cfg(test)]
#[path = "messenger_send_message_tests.rs"]
mod tests;
