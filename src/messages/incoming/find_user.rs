use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FindUser;

impl IncomingEvent for FindUser {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(username) = request.get_argument_with(0, "\t") else {
            return;
        };

        if !username.is_empty() {
            context.record(IncomingCommand::FindUser {
                username: username.to_owned(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_find_user_command_for_first_tab_separated_name() {
        let mut context = IncomingContext::new();
        FindUser.handle(
            &mut context,
            &NettyRequest::from_content("FINDUSER alice\tignored"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::FindUser {
                username: "alice".to_owned(),
            }]
        );
    }

    #[test]
    fn ignores_empty_find_user_request() {
        let mut context = IncomingContext::new();
        FindUser.handle(&mut context, &NettyRequest::from_content("FINDUSER"));

        assert!(context.commands().is_empty());
    }
}
