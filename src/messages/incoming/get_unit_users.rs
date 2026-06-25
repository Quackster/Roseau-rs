use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GetUnitUsers;

impl IncomingEvent for GetUnitUsers {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(room_name) = request.get_argument_with(1, "/") else {
            return;
        };

        if !room_name.is_empty() {
            context.record(IncomingCommand::GetUnitUsers {
                room_name: room_name.to_owned(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_unit_members_request() {
        let mut context = IncomingContext::new();
        GetUnitUsers.handle(
            &mut context,
            &NettyRequest::from_content("GETUNITUSERS x/Lido"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::GetUnitUsers {
                room_name: "Lido".to_owned(),
            }]
        );
    }
}
