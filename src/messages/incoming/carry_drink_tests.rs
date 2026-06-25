use super::carry_drink::*;
use crate::protocol::NettyRequest;

#[test]
fn records_carry_drink_status() {
    let mut context = IncomingContext::new().carry_drink_time(180);
    CarryDrink.handle(&mut context, &NettyRequest::from_content("CarryDrink tea"));

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::RemoveRoomStatus {
                key: "dance".to_owned(),
            },
            IncomingCommand::SetRoomStatus {
                key: "carryd".to_owned(),
                value: " tea".to_owned(),
                visible: false,
                timeout: 180,
            },
            IncomingCommand::MarkRoomNeedsUpdate,
        ]
    );
}
