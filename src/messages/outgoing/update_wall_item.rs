use crate::messages::OutgoingMessage;
use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateWallItem<T> {
    item: T,
}

impl<T> UpdateWallItem<T> {
    pub fn new(item: T) -> Self {
        Self { item }
    }
}

impl<T> OutgoingMessage for UpdateWallItem<T>
where
    T: SerializableObject,
{
    fn write(&self, response: &mut NettyResponse) {
        response.init("UPDATEITEM");
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
    fn composes_update_wall_item_packet() {
        let mut response = UpdateWallItem::new(WallItem).compose();

        assert_eq!(response.get(), "#UPDATEITEM\rwall-data##");
    }
}
