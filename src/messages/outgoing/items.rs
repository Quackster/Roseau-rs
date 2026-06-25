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
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct WallItem(&'static str);

    impl SerializableObject for WallItem {
        fn serialise(&self, response: &mut NettyResponse) {
            response.append(self.0);
        }
    }

    #[test]
    fn composes_items_packet() {
        let mut response = Items::new([WallItem("a"), WallItem("b")]).compose();

        assert_eq!(response.get(), "#ITEMS\ra\\b##");
    }
}
