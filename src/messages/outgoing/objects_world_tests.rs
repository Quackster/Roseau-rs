use super::objects_world::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PassiveObject(&'static str);

impl SerializableObject for PassiveObject {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append_new_argument(self.0);
    }
}

#[test]
fn composes_objects_world_packet() {
    let mut response = ObjectsWorld::new("model_a", [PassiveObject("plant")]).compose();

    assert_eq!(response.get(), "# OBJECTS WORLD 0 model_a\rplant##");
}
