use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerRequestBuddy;

impl IncomingEvent for MessengerRequestBuddy {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(username) = request.get_argument_with(0, "\r") else {
            return;
        };

        if !username.is_empty() {
            context.record(IncomingCommand::RequestBuddy {
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
    fn records_request_buddy_command() {
        let mut context = IncomingContext::new();
        MessengerRequestBuddy.handle(
            &mut context,
            &NettyRequest::from_content("MESSENGER_REQUESTBUDDY alice\rignored"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::RequestBuddy {
                username: "alice".to_owned(),
            }]
        );
    }
}
