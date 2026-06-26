use super::*;

#[test]
fn composes_open_uimakoppi_packet() {
    let mut response = OpenUimakoppi.compose();

    assert_eq!(response.get(), "#OPEN_UIMAKOPPI##");
}
