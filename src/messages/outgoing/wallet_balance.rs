use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalletBalance {
    credits: i32,
}

impl WalletBalance {
    pub fn new(credits: i32) -> Self {
        Self { credits }
    }
}

impl OutgoingMessage for WalletBalance {
    fn write(&self, response: &mut NettyResponse) {
        response.init("WALLETBALANCE");
        response.append_new_argument(self.credits);
    }
}

#[cfg(test)]
#[path = "wallet_balance_tests.rs"]
mod tests;
