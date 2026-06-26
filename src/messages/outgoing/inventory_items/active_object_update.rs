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
#[path = "active_object_update_tests.rs"]
mod tests;
