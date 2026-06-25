use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TryFlat;

impl IncomingEvent for TryFlat {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(room_id) = request.get_argument_with(1, "/") else {
            return;
        };

        let Ok(room_id) = room_id.parse::<i32>() else {
            return;
        };

        let password = request
            .get_argument_with(2, "/")
            .unwrap_or_default()
            .to_owned();

        context.record(IncomingCommand::TryFlat { room_id, password });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_try_flat_command_with_password() {
        let mut context = IncomingContext::new();
        TryFlat.handle(
            &mut context,
            &NettyRequest::from_content("TRYFLAT /42/sesame"),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::TryFlat {
                room_id: 42,
                password: "sesame".to_owned(),
            }]
        );
    }

    #[test]
    fn records_try_flat_command_without_password() {
        let mut context = IncomingContext::new();
        TryFlat.handle(&mut context, &NettyRequest::from_content("TRYFLAT /42"));

        assert_eq!(
            context.commands(),
            &[IncomingCommand::TryFlat {
                room_id: 42,
                password: String::new(),
            }]
        );
    }
}
