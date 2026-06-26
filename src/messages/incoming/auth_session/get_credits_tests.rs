use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_get_credits_command() {
    let mut context = IncomingContext::new().credits(42);
    GetCredits.handle(&mut context, &NettyRequest::from_content("GETCREDITS"));

    assert_eq!(context.commands(), &[IncomingCommand::GetCredits]);
}
