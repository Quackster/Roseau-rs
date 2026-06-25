use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GetOrderInfo;

impl IncomingEvent for GetOrderInfo {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let body = request.get_message_body();
        if body.chars().count() < 4 {
            return;
        }

        let call_id = body.chars().skip(4).collect::<String>();
        if !call_id.is_empty() {
            context.record(IncomingCommand::GetOrderInfo { call_id });
        }
    }
}

#[cfg(test)]
#[path = "get_order_info_tests.rs"]
mod tests;
