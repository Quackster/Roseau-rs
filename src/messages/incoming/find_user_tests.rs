use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_find_user_command_for_first_tab_separated_name() {
    let mut context = IncomingContext::new();
    FindUser.handle(
        &mut context,
        &NettyRequest::from_content("FINDUSER alice\tignored"),
    );

    assert_eq!(
        context.commands(),
        &[IncomingCommand::FindUser {
            username: "alice".to_owned(),
        }]
    );
}

#[test]
fn ignores_empty_find_user_request() {
    let mut context = IncomingContext::new();
    FindUser.handle(&mut context, &NettyRequest::from_content("FINDUSER"));

    assert!(context.commands().is_empty());
}
