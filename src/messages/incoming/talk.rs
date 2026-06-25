use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;
use crate::util::filter_input;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Talk;

impl IncomingEvent for Talk {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        context.record(IncomingCommand::ResetAfkTimer);

        if !context.is_in_room() || request.get_argument_amount() < 1 {
            return;
        }

        let mode = request.get_header();
        if !matches!(mode, "CHAT" | "SHOUT" | "WHISPER") {
            return;
        }

        let message = filter_input(&request.get_message_body());
        if message.is_empty() {
            return;
        }

        context.record(IncomingCommand::Talk {
            mode: mode.to_owned(),
            message,
        });
    }
}
