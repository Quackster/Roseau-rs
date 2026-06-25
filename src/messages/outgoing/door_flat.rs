use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoorFlat {
    item_id: i32,
    room_id: i32,
}

impl DoorFlat {
    pub fn new(item_id: i32, room_id: i32) -> Self {
        Self { item_id, room_id }
    }
}

impl OutgoingMessage for DoorFlat {
    fn write(&self, response: &mut NettyResponse) {
        response.init("DOORFLAT");
        response.append_new_argument(self.item_id);
        response.append_new_argument(self.room_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_door_flat_packet() {
        let mut response = DoorFlat::new(12, 34).compose();

        assert_eq!(response.get(), "#DOORFLAT\r12\r34##");
    }
}
