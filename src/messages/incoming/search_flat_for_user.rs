use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SearchFlatForUser;

impl IncomingEvent for SearchFlatForUser {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(username) = request.get_argument_with(1, "/") else {
            return;
        };

        if !username.is_empty() {
            context.record(IncomingCommand::SearchFlatForUser {
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
    fn records_user_room_search() {
        let mut context = IncomingContext::new();
        SearchFlatForUser.handle(
            &mut context,
            &NettyRequest::from_content("SEARCHFLATFORUSER /alice"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::SearchFlatForUser {
                username: "alice".to_owned(),
            }]
        );
    }
}
