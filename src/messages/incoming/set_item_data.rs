use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SetItemData;

impl IncomingEvent for SetItemData {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(item_id) = request.get_argument_with(1, "/") else {
            return;
        };

        let Ok(item_id) = item_id.parse::<i32>() else {
            return;
        };

        let data = request
            .get_message_body()
            .replace(&format!("/{item_id}/"), "");

        context.record(IncomingCommand::SetItemData { item_id, data });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_set_item_data_command() {
        let mut context = IncomingContext::new();
        SetItemData.handle(
            &mut context,
            &NettyRequest::from_content("SETITEMDATA /42/FFFF31 note"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::SetItemData {
                item_id: 42,
                data: "FFFF31 note".to_owned(),
            }]
        );
    }
}
