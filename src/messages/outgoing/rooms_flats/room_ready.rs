use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomReady {
    description: String,
}

impl RoomReady {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
        }
    }
}

impl OutgoingMessage for RoomReady {
    fn write(&self, response: &mut NettyResponse) {
        response.init("ROOM_READY");
        response.append_new_argument(&self.description);
    }
}

#[cfg(test)]
#[path = "room_ready_tests.rs"]
mod tests;
