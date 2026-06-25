use crate::messages::OutgoingMessage;
use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectsWorld<T> {
    model: String,
    passive_objects: Vec<T>,
}

impl<T> ObjectsWorld<T> {
    pub fn new(model: impl Into<String>, passive_objects: impl IntoIterator<Item = T>) -> Self {
        Self {
            model: model.into(),
            passive_objects: passive_objects.into_iter().collect(),
        }
    }
}

impl<T> OutgoingMessage for ObjectsWorld<T>
where
    T: SerializableObject,
{
    fn write(&self, response: &mut NettyResponse) {
        response.init(" OBJECTS WORLD 0");
        response.append_argument(&self.model);

        for item in &self.passive_objects {
            response.append_object(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
