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
#[path = "set_stuff_data_tests.rs"]
mod tests;
