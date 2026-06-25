use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JumpPerformance;

impl IncomingEvent for JumpPerformance {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(data) = request.get_argument_with(3, "\r") else {
            return;
        };

        context.record(IncomingCommand::JumpPerformance {
            data: data.to_owned(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_jump_performance_data() {
        let mut context = IncomingContext::new();
        JumpPerformance.handle(
            &mut context,
            &NettyRequest::from_content("JUMPPERF a\rb\rc\rdive"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::JumpPerformance {
                data: "dive".to_owned(),
            }]
        );
    }
}
