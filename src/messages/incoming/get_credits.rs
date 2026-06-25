use crate::messages::outgoing::{MessengerSmsAccount, MessengersReady, WalletBalance};
use crate::messages::{IncomingContext, IncomingEvent, OutgoingMessage};
use crate::protocol::ClientMessage;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GetCredits;

impl IncomingEvent for GetCredits {
    fn handle(&self, context: &mut IncomingContext, _request: &dyn ClientMessage) {
        context.send(WalletBalance::new(context.credits_value()).compose());
        context.send(MessengerSmsAccount.compose());
        context.send(MessengersReady.compose());
    }
}
