use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_kick_user_command() {
    let mut context = IncomingContext::new();
    KillUser.handle(&mut context, &NettyRequest::from_content("KILLUSER bob"));

    assert_eq!(
        context.commands(),
        &[
            IncomingCommand::ResetAfkTimer,
            IncomingCommand::KickUser {
                username: "bob".to_owned(),
            }
        ]
    );
}
