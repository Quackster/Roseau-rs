use super::*;
use crate::protocol::NettyRequest;

#[test]
fn removes_dance_status_when_requested() {
    let mut context = IncomingContext::new().in_room(true);
    Stop.handle(&mut context, &NettyRequest::from_content("STOP Dance"));

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::RemoveRoomStatus {
                key: "dance".to_owned(),
            },
            IncomingCommand::MarkRoomNeedsUpdate,
        ]
    );
}
