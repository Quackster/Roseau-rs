use super::wallet_balance::*;

#[test]
fn composes_wallet_balance_packet() {
    let mut response = WalletBalance::new(42).compose();

    assert_eq!(response.get(), "#WALLETBALANCE\r42##");
}
