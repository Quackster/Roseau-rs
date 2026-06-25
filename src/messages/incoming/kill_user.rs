use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KillUser;

impl IncomingEvent for KillUser {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let username = request.get_message_body();
        context.record(IncomingCommand::ResetAfkTimer);

        if !username.is_empty() {
            context.record(IncomingCommand::KickUser { username });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_kick_user_command() {
        let mut context = IncomingContext::new();
        KillUser.handle(&mut context, &NettyRequest::from_content("KILLUSER bob"));

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::ResetAfkTimer,
                IncomingCommand::KickUser {
                    username: "bob".to_owned(),
                }
            ]
        );
    }
}
