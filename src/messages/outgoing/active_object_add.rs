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
#[path = "active_object_add_tests.rs"]
mod tests;
