use super::active_objects::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct FloorItem(&'static str);

impl SerializableObject for FloorItem {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append_new_argument(self.0);
    }
}

#[test]
fn composes_active_objects_packet() {
    let mut response = ActiveObjects::new([FloorItem("chair"), FloorItem("table")]).compose();

    assert_eq!(response.get(), "#ACTIVE_OBJECTS\rchair\rtable##");
}
