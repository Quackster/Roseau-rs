use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SetStripItemData;

impl IncomingEvent for SetStripItemData {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(item_id) = request.get_argument_with(1, "\r") else {
            return;
        };

        let Ok(item_id) = item_id.parse::<i32>() else {
            return;
        };

        context.record(IncomingCommand::UseStripItem { item_id });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_use_strip_item_command() {
        let mut context = IncomingContext::new();
        SetStripItemData.handle(
            &mut context,
            &NettyRequest::from_content("SETSTRIPITEMDATA ignored\r42"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::UseStripItem { item_id: 42 }]
        );
    }
}
