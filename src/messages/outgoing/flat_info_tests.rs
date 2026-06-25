use super::flat_info::*;

#[test]
fn composes_flat_info_packet() {
    let mut response = FlatInfo::new(42).compose();

    assert_eq!(response.get(), "#SETFLATINFO\r/42/##");
}
