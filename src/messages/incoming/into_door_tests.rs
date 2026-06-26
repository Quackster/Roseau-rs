use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_enter_door_command() {
    let mut context = IncomingContext::new().in_room(true);
    IntoDoor.handle(&mut context, &NettyRequest::from_content("IntoDoor 77"));

    assert_eq!(
        context.commands(),
        &[IncomingCommand::EnterDoor { item_id: 77 }]
    );
}

#[test]
fn ignores_door_items_outside_rooms() {
    let mut context = IncomingContext::new();
    IntoDoor.handle(&mut context, &NettyRequest::from_content("IntoDoor 77"));

    assert!(context.commands().is_empty());
}

#[test]
fn ignores_non_numeric_item_ids() {
    let mut context = IncomingContext::new().in_room(true);
    IntoDoor.handle(&mut context, &NettyRequest::from_content("IntoDoor nope"));

    assert!(context.commands().is_empty());
}
