use super::*;

#[test]
fn composes_messenger_sms_account_packet() {
    let mut response = MessengerSmsAccount.compose();

    assert_eq!(response.get(), "#MESSENGERSMSACCOUNT\rnoaccount##");
}
