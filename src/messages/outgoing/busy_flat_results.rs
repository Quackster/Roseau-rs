use crate::messages::OutgoingMessage;
use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BusyFlatResults<T> {
    rooms: Vec<T>,
}

impl<T> BusyFlatResults<T> {
    pub fn new(rooms: impl IntoIterator<Item = T>) -> Self {
        Self {
            rooms: rooms.into_iter().collect(),
        }
    }
}

impl<T> OutgoingMessage for BusyFlatResults<T>
where
    T: SerializableObject,
{
    fn write(&self, response: &mut NettyResponse) {
        response.init("BUSY_FLAT_RESULTS 1");

        for room in &self.rooms {
            response.append_object(room);
        }
    }
}
