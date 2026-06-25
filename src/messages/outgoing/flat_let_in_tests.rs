use super::flat_let_in::*;

#[test]
fn composes_flat_let_in_packet() {
    let mut response = FlatLetIn.compose();

    assert_eq!(response.get(), "#FLAT_LETIN##");
}
