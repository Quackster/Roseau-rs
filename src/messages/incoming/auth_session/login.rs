use crate::messages::outgoing::Error;
use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent, OutgoingMessage};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Login;

impl IncomingEvent for Login {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let (Some(username), Some(password)) = (request.get_argument(0), request.get_argument(1))
        else {
            context.send(Error::new("Login incorrect").compose());
            return;
        };

        context.record(IncomingCommand::Login {
            username: username.to_owned(),
            password: password.to_owned(),
            // Java: the room branch only runs when argumentAmount > 2,
            // regardless of which port the login arrived on.
            room_login: request.get_argument_amount() > 2,
        });
    }
}

#[cfg(test)]
#[path = "login_tests.rs"]
mod tests;
