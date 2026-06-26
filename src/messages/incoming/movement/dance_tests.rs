use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_dance_status_when_in_room() {
    let mut context = IncomingContext::new().in_room(true);
    Dance.handle(&mut context, &NettyRequest::from_content("Dance"));

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::SetRoomStatus {
                key: "dance".to_owned(),
                value: String::new(),
                visible: true,
                timeout: -1,
            },
            IncomingCommand::MarkRoomNeedsUpdate,
        ]
    );
}
