use crate::game::player::PlayerNameApproval;
use crate::messages::{IncomingContext, IncomingEvent, OutgoingMessage};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ApproveName;

impl IncomingEvent for ApproveName {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(name) = request.get_argument(0) else {
            return;
        };

        if name.is_empty() {
            return;
        }

        let approval = PlayerNameApproval::evaluate(name, context.username_chars_value());
        if let Some(packet) = approval.name_approved() {
            context.send(packet.compose());
        }
        if let Some(packet) = approval.name_unacceptable() {
            context.send(packet.compose());
        }
    }
}

#[cfg(test)]
#[path = "approve_name_tests.rs"]
mod tests;
