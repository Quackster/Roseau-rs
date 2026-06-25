use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoorIn {
    item_padding: String,
    item_id: i32,
    player: String,
}

impl DoorIn {
    pub fn new(item_padding: impl Into<String>, item_id: i32, player: impl Into<String>) -> Self {
        Self {
            item_padding: item_padding.into(),
            item_id,
            player: player.into(),
        }
    }
}

impl OutgoingMessage for DoorIn {
    fn write(&self, response: &mut NettyResponse) {
        response.init("DOOR_IN");
        response.append_new_argument(&self.item_padding);
        response.append(self.item_id);
        response.append_part_argument(&self.player);
    }
}

#[cfg(test)]
#[path = "door_in_tests.rs"]
mod tests;
