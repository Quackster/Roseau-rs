use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SetStuffData;

impl IncomingEvent for SetStuffData {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let (Some(item_id), Some(data_class), Some(custom_data)) = (
            request.get_argument_with(1, "/"),
            request.get_argument_with(2, "/"),
            request.get_argument_with(3, "/"),
        ) else {
            return;
        };

        let Ok(item_id) = item_id.parse::<i32>() else {
            return;
        };

        context.record(IncomingCommand::SetStuffData {
            item_id,
            data_class: data_class.to_owned(),
            custom_data: custom_data.to_owned(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_set_stuff_data_command() {
        let mut context = IncomingContext::new();
        SetStuffData.handle(
            &mut context,
            &NettyRequest::from_content("SETSTUFFDATA /42/state/open"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::SetStuffData {
                item_id: 42,
                data_class: "state".to_owned(),
                custom_data: "open".to_owned(),
            }]
        );
    }
}
