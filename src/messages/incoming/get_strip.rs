use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GetStrip;

impl IncomingEvent for GetStrip {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        context.record(IncomingCommand::RefreshInventory {
            category: request.get_message_body(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_inventory_refresh() {
        let mut context = IncomingContext::new();
        GetStrip.handle(&mut context, &NettyRequest::from_content("GETSTRIP floor"));

        assert_eq!(
            context.commands(),
            &[IncomingCommand::RefreshInventory {
                category: "floor".to_owned(),
            }]
        );
    }
}
