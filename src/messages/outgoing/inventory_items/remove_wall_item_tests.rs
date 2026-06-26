use super::*;

#[test]
fn composes_remove_wall_item_packet() {
    let mut response = RemoveWallItem::new(55).compose();

    assert_eq!(response.get(), "#REMOVEITEM\r55##");
}
