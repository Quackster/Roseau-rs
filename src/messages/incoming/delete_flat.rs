use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DeleteFlat;

impl IncomingEvent for DeleteFlat {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(room_id) = request.get_argument_with(1, "/") else {
            return;
        };

        let Ok(room_id) = room_id.parse::<i32>() else {
            return;
        };

        context.record(IncomingCommand::DeleteFlat { room_id });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_delete_flat_command() {
        let mut context = IncomingContext::new();
        DeleteFlat.handle(&mut context, &NettyRequest::from_content("DELETEFLAT x/99"));

        assert_eq!(
            context.commands(),
            &[IncomingCommand::DeleteFlat { room_id: 99 }]
        );
    }
}
