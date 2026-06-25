use crate::messages::{IncomingCommand, IncomingContext, IncomingEvent};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InfoRetrieve;

impl IncomingEvent for InfoRetrieve {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        if context.is_authenticated() {
            context.record(IncomingCommand::RetrieveUserInfo);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn records_user_info_request_when_authenticated() {
        let mut context = IncomingContext::new().authenticated(true);
        InfoRetrieve.handle(&mut context, &NettyRequest::from_content("INFORETRIEVE"));

        assert_eq!(context.commands(), &[IncomingCommand::RetrieveUserInfo]);
    }

    #[test]
    fn ignores_user_info_request_when_unauthenticated() {
        let mut context = IncomingContext::new();
        InfoRetrieve.handle(&mut context, &NettyRequest::from_content("INFORETRIEVE"));

        assert!(context.commands().is_empty());
    }
}
