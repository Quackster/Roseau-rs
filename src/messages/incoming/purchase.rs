use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Purchase;

impl IncomingEvent for Purchase {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let call_id = request.get_message_body().replace('/', "");
        if call_id.is_empty() {
            return;
        }

        context.record(IncomingCommand::Purchase { call_id });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_purchase_call_id_without_slashes() {
        let mut context = IncomingContext::new();
        Purchase.handle(
            &mut context,
            &NettyRequest::from_content("PURCHASE /chair_blue"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::Purchase {
                call_id: "chair_blue".to_owned(),
            }]
        );
    }
}
