use crate::messages::OutgoingMessage;
use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveObjectUpdate<T> {
    item: Option<T>,
}

impl<T> ActiveObjectUpdate<T> {
    pub fn new(item: Option<T>) -> Self {
        Self { item }
    }
}

impl<T> OutgoingMessage for ActiveObjectUpdate<T>
where
    T: SerializableObject,
{
    fn write(&self, response: &mut NettyResponse) {
        response.init("ACTIVEOBJECT_UPDATE");

        if let Some(item) = &self.item {
            response.append_object(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
