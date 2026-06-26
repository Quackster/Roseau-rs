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
fn records_empty_find_user_request_for_missing_user_response() {
    let mut context = IncomingContext::new();
    FindUser.handle(&mut context, &NettyRequest::from_content("FINDUSER"));

    assert_eq!(
        context.commands(),
        &[IncomingCommand::FindUser {
            username: String::new(),
        }]
    );
}
