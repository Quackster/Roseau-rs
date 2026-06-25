use crate::messages::OutgoingMessage;
use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveObjects<T> {
    items: Vec<T>,
}

impl<T> ActiveObjects<T> {
    pub fn new(items: impl IntoIterator<Item = T>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

impl<T> OutgoingMessage for ActiveObjects<T>
where
    T: SerializableObject,
{
    fn write(&self, response: &mut NettyResponse) {
        response.init("ACTIVE_OBJECTS");

        for item in &self.items {
            response.append_object(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
