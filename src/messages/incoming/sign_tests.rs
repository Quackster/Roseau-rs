use super::sign::*;
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
