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
