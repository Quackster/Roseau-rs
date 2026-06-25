use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct WallItem;

impl SerializableObject for WallItem {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append("wall-data");
    }
}

#[test]
fn composes_update_wall_item_packet() {
    let mut response = UpdateWallItem::new(WallItem).compose();

    assert_eq!(response.get(), "#UPDATEITEM\rwall-data##");
}
