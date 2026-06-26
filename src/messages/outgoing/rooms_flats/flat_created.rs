use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatCreated {
    room_id: i32,
    room_name: String,
}

impl FlatCreated {
    pub fn new(room_id: i32, room_name: impl Into<String>) -> Self {
        Self {
            room_id,
            room_name: room_name.into(),
        }
    }
}

impl OutgoingMessage for FlatCreated {
    fn write(&self, response: &mut NettyResponse) {
        response.init("FLATCREATED");
        response.append_new_argument(self.room_id);
        response.append_argument(&self.room_name);
    }
}

#[cfg(test)]
#[path = "flat_created_tests.rs"]
mod tests;
