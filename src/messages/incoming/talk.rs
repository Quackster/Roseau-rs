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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_chat_message_when_in_room() {
        let mut context = IncomingContext::new().in_room(true);
        Talk.handle(
            &mut context,
            &NettyRequest::from_content("CHAT hello\rthere"),
        );

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::ResetAfkTimer,
                IncomingCommand::Talk {
                    mode: "CHAT".to_owned(),
                    message: "hello there".to_owned(),
                }
            ]
        );
    }

    #[test]
    fn ignores_chat_message_outside_room() {
        let mut context = IncomingContext::new();
        Talk.handle(&mut context, &NettyRequest::from_content("CHAT hello"));

        assert_eq!(context.commands(), &[IncomingCommand::ResetAfkTimer]);
    }
}
