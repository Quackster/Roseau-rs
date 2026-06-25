use super::assign_rights::*;
use crate::protocol::NettyRequest;

#[test]
fn records_assign_rights_command() {
    let mut context = IncomingContext::new();
    AssignRights.handle(
        &mut context,
        &NettyRequest::from_content("ASSIGNRIGHTS alice"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::AssignRights {
            username: "alice".to_owned(),
        }]
    );
}
