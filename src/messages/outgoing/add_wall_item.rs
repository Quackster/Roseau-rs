use crate::messages::OutgoingMessage;
use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddWallItem<T> {
    item: T,
}

impl<T> AddWallItem<T> {
    pub fn new(item: T) -> Self {
        Self { item }
    }
}

impl<T> OutgoingMessage for AddWallItem<T>
where
    T: SerializableObject,
{
    fn write(&self, response: &mut NettyResponse) {
        response.init("ADDITEM");
        response.append('\r');
        response.append_object(&self.item);
    }
}
