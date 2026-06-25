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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct WallItem;

    impl SerializableObject for WallItem {
        fn serialise(&self, response: &mut NettyResponse) {
            response.append("wall-data");
        }
    }

    #[test]
    fn composes_add_wall_item_packet() {
        let mut response = AddWallItem::new(WallItem).compose();

        assert_eq!(response.get(), "#ADDITEM\rwall-data##");
    }
}
