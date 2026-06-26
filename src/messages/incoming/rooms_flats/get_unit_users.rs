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
#[path = "get_unit_users_tests.rs"]
mod tests;
