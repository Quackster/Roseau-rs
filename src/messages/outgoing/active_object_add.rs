use crate::messages::OutgoingMessage;
use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveObjectAdd<T> {
    item: T,
}

impl<T> ActiveObjectAdd<T> {
    pub fn new(item: T) -> Self {
        Self { item }
    }
}

impl<T> OutgoingMessage for ActiveObjectAdd<T>
where
    T: SerializableObject,
{
    fn write(&self, response: &mut NettyResponse) {
        response.init("ACTIVEOBJECT_ADD");
        response.append_object(&self.item);
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
    fn composes_active_object_add_packet() {
        let mut response = ActiveObjectAdd::new(Item).compose();

        assert_eq!(response.get(), "#ACTIVEOBJECT_ADD chair##");
    }
}
