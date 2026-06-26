use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_let_user_in_command() {
    let mut context = IncomingContext::new();
    LetUserIn.handle(&mut context, &NettyRequest::from_content("LETUSERIN alice"));

    assert_eq!(
        context.commands(),
        &[IncomingCommand::LetUserIn {
            username: "alice".to_owned(),
        }]
    );
}
