use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_unit_members_request() {
    let mut context = IncomingContext::new();
    GetUnitUsers.handle(
        &mut context,
        &NettyRequest::from_content("GETUNITUSERS x/Lido"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::GetUnitUsers {
            room_name: "Lido".to_owned(),
        }]
    );
}
