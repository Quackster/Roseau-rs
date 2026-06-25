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
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_catalogue_call_id() {
        let mut context = IncomingContext::new();
        GetOrderInfo.handle(
            &mut context,
            &NettyRequest::from_content("GETORDERINFO xxx chair"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::GetOrderInfo {
                call_id: "chair".to_owned(),
            }]
        );
    }
}
