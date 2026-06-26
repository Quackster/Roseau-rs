use crate::messages::OutgoingMessage;
use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items<T> {
    wall_items: Vec<T>,
}

impl<T> Items<T> {
    pub fn new(wall_items: impl IntoIterator<Item = T>) -> Self {
        Self {
            wall_items: wall_items.into_iter().collect(),
        }
    }
}

impl<T> OutgoingMessage for Items<T>
where
    T: SerializableObject,
{
    fn write(&self, response: &mut NettyResponse) {
        response.init("ITEMS");
        response.append('\r');

        for (index, item) in self.wall_items.iter().enumerate() {
            response.append_object(item);

            if index + 1 != self.wall_items.len() {
                response.append("\\");
            }
        }
    }
}

#[cfg(test)]
#[path = "items_tests.rs"]
mod tests;
