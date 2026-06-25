use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemoveWallItem {
    item_id: i32,
}

impl RemoveWallItem {
    pub fn new(item_id: i32) -> Self {
        Self { item_id }
    }
}

impl OutgoingMessage for RemoveWallItem {
    fn write(&self, response: &mut NettyResponse) {
        response.init("REMOVEITEM");
        response.append_new_argument(self.item_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_remove_wall_item_packet() {
        let mut response = RemoveWallItem::new(55).compose();

        assert_eq!(response.get(), "#REMOVEITEM\r55##");
    }
}
