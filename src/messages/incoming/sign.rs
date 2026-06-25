use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Sign;

impl IncomingEvent for Sign {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        if !context.is_in_room_model("pool_b") {
            return;
        }

        let Ok(sign_id) = request.get_message_body().parse::<i32>() else {
            return;
        };

        if !(1..=14).contains(&sign_id) {
            return;
        }

        context.record(IncomingCommand::RemoveRoomStatus {
            key: "dance".to_owned(),
        });
        context.record(IncomingCommand::SetRoomStatus {
            key: "sign".to_owned(),
            value: format!(" {sign_id}"),
            visible: false,
            timeout: 2,
        });
        context.record(IncomingCommand::MarkRoomNeedsUpdate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_sign_status_when_in_range() {
        let mut context = IncomingContext::new().room_model_name("pool_b");
        Sign.handle(&mut context, &NettyRequest::from_content("Sign 7"));

        assert_eq!(
            context.commands(),
            &[
                IncomingCommand::RemoveRoomStatus {
                    key: "dance".to_owned(),
                },
                IncomingCommand::SetRoomStatus {
                    key: "sign".to_owned(),
                    value: " 7".to_owned(),
                    visible: false,
                    timeout: 2,
                },
                IncomingCommand::MarkRoomNeedsUpdate,
            ]
        );
    }

    #[test]
    fn ignores_signs_outside_the_diving_room() {
        let mut context = IncomingContext::new().room_model_name("guest_room");
        Sign.handle(&mut context, &NettyRequest::from_content("Sign 7"));

        assert!(context.commands().is_empty());
    }
}
