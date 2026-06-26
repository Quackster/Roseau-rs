use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item;

impl SerializableObject for Item {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append_argument("chair");
    }
}

#[test]
fn composes_active_object_add_packet() {
    let mut response = ActiveObjectAdd::new(Item).compose();

    assert_eq!(response.get(), "#ACTIVEOBJECT_ADD chair##");
}
