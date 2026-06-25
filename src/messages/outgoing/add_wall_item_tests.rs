use super::add_wall_item::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct WallItem;

impl SerializableObject for WallItem {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append("wall-data");
    }
}

#[test]
fn composes_add_wall_item_packet() {
    let mut response = AddWallItem::new(WallItem).compose();

    assert_eq!(response.get(), "#ADDITEM\rwall-data##");
}
