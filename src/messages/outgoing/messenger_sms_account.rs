use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerSmsAccount;

impl OutgoingMessage for MessengerSmsAccount {
    fn write(&self, response: &mut NettyResponse) {
        response.init("MESSENGERSMSACCOUNT");
        response.append_new_argument("noaccount");
    }
}

#[cfg(test)]
#[path = "messenger_sms_account_tests.rs"]
mod tests;
