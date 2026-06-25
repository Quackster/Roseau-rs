use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AssignRights;

impl IncomingEvent for AssignRights {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let username = request.get_message_body();
        if !username.is_empty() {
            context.record(IncomingCommand::AssignRights { username });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_assign_rights_command() {
        let mut context = IncomingContext::new();
        AssignRights.handle(
            &mut context,
            &NettyRequest::from_content("ASSIGNRIGHTS alice"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::AssignRights {
                username: "alice".to_owned(),
            }]
        );
    }
}
