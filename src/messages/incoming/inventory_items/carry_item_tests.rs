use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_carry_status() {
    let mut context = IncomingContext::new().carry_drink_time(180);
    CarryItem.handle(
        &mut context,
        &NettyRequest::from_content("CarryItem cola/bottle"),
    );

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::RemoveRoomStatus {
                key: "dance".to_owned(),
            },
            IncomingCommand::SetRoomStatus {
                key: "carryd".to_owned(),
                value: " cola?bottle".to_owned(),
                visible: false,
                timeout: 180,
            },
            IncomingCommand::MarkRoomNeedsUpdate,
        ]
    );
}
