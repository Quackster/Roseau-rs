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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::NettyRequest;

    #[test]
    fn sends_wallet_and_messenger_bootstrap_packets() {
        let mut context = IncomingContext::new().credits(42);
        GetCredits.handle(&mut context, &NettyRequest::from_content("GETCREDITS"));

        let mut wallet = context.sent()[0].clone();
        let mut sms = context.sent()[1].clone();
        let mut ready = context.sent()[2].clone();

        assert_eq!(wallet.get(), "#WALLETBALANCE\r42##");
        assert_eq!(sms.get(), "#MESSENGERSMSACCOUNT\rnoaccount##");
        assert_eq!(ready.get(), "#MESSENGERSREADY##");
    }
}
