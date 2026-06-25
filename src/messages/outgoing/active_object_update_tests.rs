use super::active_object_update::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item;

impl SerializableObject for Item {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append_argument("chair");
    }
}

#[test]
fn composes_active_object_update_packet() {
    let mut response = ActiveObjectUpdate::new(Some(Item)).compose();

    assert_eq!(response.get(), "#ACTIVEOBJECT_UPDATE chair##");
}
