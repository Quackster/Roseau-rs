use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SetFlatInfo;

impl IncomingEvent for SetFlatInfo {
    fn handle(&self, context: &mut IncomingContext, request: &dyn ClientMessage) {
        let Some(room_id) = request.get_argument_with(1, "/") else {
            return;
        };

        let Ok(room_id) = room_id.parse::<i32>() else {
            return;
        };

        let fields = request
            .get_message_body()
            .replace(&format!("/{room_id}/"), "");

        let Some(description) = field_value(&fields, 0, "description=") else {
            return;
        };
        let Some(password) = field_value(&fields, 1, "password=") else {
            return;
        };
        let Some(all_super_user) = field_value(&fields, 2, "allsuperuser=") else {
            return;
        };

        context.record(IncomingCommand::SetFlatInfo {
            room_id,
            description,
            password,
            all_super_user: all_super_user == "1",
        });
    }
}

fn field_value(fields: &str, index: usize, prefix: &str) -> Option<String> {
    fields
        .split('\r')
        .nth(index)
        .and_then(|field| field.strip_prefix(prefix))
        .map(ToOwned::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_flat_info_update() {
        let mut context = IncomingContext::new();
        SetFlatInfo.handle(
            &mut context,
            &NettyRequest::from_content(
                "SETFLATINFO /36/description=hello\rpassword=secret\rallsuperuser=1",
            ),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::SetFlatInfo {
                room_id: 36,
                description: "hello".to_owned(),
                password: "secret".to_owned(),
                all_super_user: true,
            }]
        );
    }

    #[test]
    fn preserves_slashes_inside_flat_info_fields() {
        let mut context = IncomingContext::new();
        SetFlatInfo.handle(
            &mut context,
            &NettyRequest::from_content(
                "SETFLATINFO /36/description=hello/world\rpassword=a/b\rallsuperuser=0",
            ),
        );

        assert_eq!(
            context.commands(),
            &[IncomingCommand::SetFlatInfo {
                room_id: 36,
                description: "hello/world".to_owned(),
                password: "a/b".to_owned(),
                all_super_user: false,
            }]
        );
    }
}
