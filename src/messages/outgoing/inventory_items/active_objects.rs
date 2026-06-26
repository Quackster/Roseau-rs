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
#[path = "active_objects_tests.rs"]
mod tests;
