use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RemoveRights;

impl IncomingEvent for RemoveRights {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let username = request.get_message_body();
        if !username.is_empty() {
            context.record(IncomingCommand::RemoveRights { username });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_remove_rights_command() {
        let mut context = IncomingContext::new();
        RemoveRights.handle(
            &mut context,
            &NettyRequest::from_content("REMOVERIGHTS alice"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::RemoveRights {
                username: "alice".to_owned(),
            }]
        );
    }
}
