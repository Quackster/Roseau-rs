use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlatInfo {
    room_id: i32,
}

impl FlatInfo {
    pub fn new(room_id: i32) -> Self {
        Self { room_id }
    }
}

impl OutgoingMessage for FlatInfo {
    fn write(&self, response: &mut NettyResponse) {
        response.init("SETFLATINFO");
        response.append_new_argument("/");
        response.append(self.room_id);
        response.append("/");
    }
}

#[cfg(test)]
#[path = "flat_info_tests.rs"]
mod tests;
