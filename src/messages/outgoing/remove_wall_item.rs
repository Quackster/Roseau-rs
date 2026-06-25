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
