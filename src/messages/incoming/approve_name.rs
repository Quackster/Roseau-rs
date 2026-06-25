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
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn sends_name_approved_for_allowed_name() {
        let mut context =
            IncomingContext::new().username_chars("abcdefghijklmnopqrstuvwxyz0123456789");
        ApproveName.handle(
            &mut context,
            &NettyRequest::from_content("APPROVENAME alice1"),
        );
        let mut response = context.sent()[0].clone();

        assert_eq!(response.get(), "#NAME_APPROVED##");
    }

    #[test]
    fn sends_name_unacceptable_for_reserved_prefix() {
        let mut context = IncomingContext::new();
        ApproveName.handle(
            &mut context,
            &NettyRequest::from_content("APPROVENAME MOD-alice"),
        );
        let mut response = context.sent()[0].clone();

        assert_eq!(response.get(), "#NAME_UNACCEPTABLE##");
    }
}
