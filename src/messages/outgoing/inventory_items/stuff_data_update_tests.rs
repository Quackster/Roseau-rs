use super::*;

#[test]
fn composes_stuff_data_update_packet() {
    let mut response = StuffDataUpdate::new("i:", 7, "poster", "blue").compose();

    assert_eq!(response.get(), "#STUFFDATAUPDATE\ri:7//poster/blue##");
}
