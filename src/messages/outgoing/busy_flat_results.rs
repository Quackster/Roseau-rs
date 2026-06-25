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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RoomSummary(&'static str);

    impl SerializableObject for RoomSummary {
        fn serialise(&self, response: &mut NettyResponse) {
            response.append_new_argument(self.0);
        }
    }

    #[test]
    fn composes_busy_flat_results_packet() {
        let mut response =
            BusyFlatResults::new([RoomSummary("lobby"), RoomSummary("cafe")]).compose();

        assert_eq!(response.get(), "#BUSY_FLAT_RESULTS 1\rlobby\rcafe##");
    }
}
