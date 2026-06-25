use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LetUserIn;

impl IncomingEvent for LetUserIn {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let username = request.get_message_body();
        if !username.is_empty() {
            context.record(IncomingCommand::LetUserIn { username });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_let_user_in_command() {
        let mut context = IncomingContext::new();
        LetUserIn.handle(&mut context, &NettyRequest::from_content("LETUSERIN alice"));

        assert_eq!(
            context.commands(),
            &[IncomingCommand::LetUserIn {
                username: "alice".to_owned(),
            }]
        );
    }
}
