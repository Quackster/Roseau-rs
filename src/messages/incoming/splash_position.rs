use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SplashPosition;

impl IncomingEvent for SplashPosition {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        if !context.is_in_room() {
            return;
        }

        let position = request.get_message_body();
        if position.is_empty() {
            return;
        }

        context.record(IncomingCommand::SplashPosition { position });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_splash_position_when_in_room() {
        let mut context = IncomingContext::new().in_room(true);
        SplashPosition.handle(
            &mut context,
            &NettyRequest::from_content("SPLASHPOSITION 17,18,0.0"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::SplashPosition {
                position: "17,18,0.0".to_owned(),
            }]
        );
    }

    #[test]
    fn ignores_splash_position_outside_room() {
        let mut context = IncomingContext::new();
        SplashPosition.handle(
            &mut context,
            &NettyRequest::from_content("SPLASHPOSITION 17,18,0.0"),
        );

        assert!(context.commands().is_empty());
    }
}
