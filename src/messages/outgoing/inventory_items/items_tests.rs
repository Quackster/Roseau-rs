use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct WallItem(&'static str);

impl SerializableObject for WallItem {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append(self.0);
    }
}

#[test]
fn composes_items_packet() {
    let mut response = Items::new([WallItem("a"), WallItem("b")]).compose();

    assert_eq!(response.get(), "#ITEMS\ra\\b##");
}
