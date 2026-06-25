use super::get_flat_info::*;
use crate::protocol::NettyRequest;

#[test]
fn records_get_flat_info_command() {
    let mut context = IncomingContext::new();
    GetFlatInfo.handle(
        &mut context,
        &NettyRequest::from_content("GETFLATINFO x/99"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::GetFlatInfo { room_id: 99 }]
    );
}
