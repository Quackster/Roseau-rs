use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_remove_rights_command() {
    let mut context = IncomingContext::new();
    RemoveRights.handle(
        &mut context,
        &NettyRequest::from_content("REMOVERIGHTS alice"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::RemoveRights {
            username: "alice".to_owned(),
        }]
    );
}
