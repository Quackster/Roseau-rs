use super::*;
use crate::protocol::NettyRequest;

#[test]
fn records_user_info_request_when_authenticated() {
    let mut context = IncomingContext::new();
    InfoRetrieve.handle(&mut context, &NettyRequest::from_content("INFORETRIEVE"));

    assert_eq!(context.commands(), &[IncomingCommand::RetrieveUserInfo]);
}

#[test]
fn records_user_info_request_before_runtime_authentication() {
    let mut context = IncomingContext::new();
    InfoRetrieve.handle(&mut context, &NettyRequest::from_content("INFORETRIEVE"));

    assert_eq!(context.commands(), &[IncomingCommand::RetrieveUserInfo]);
}
