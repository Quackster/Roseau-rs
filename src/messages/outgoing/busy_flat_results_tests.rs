use super::busy_flat_results::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct RoomSummary(&'static str);

impl SerializableObject for RoomSummary {
    fn serialise(&self, response: &mut NettyResponse) {
        response.append_new_argument(self.0);
    }
}

#[test]
fn composes_busy_flat_results_packet() {
    let mut response = BusyFlatResults::new([RoomSummary("lobby"), RoomSummary("cafe")]).compose();

    assert_eq!(response.get(), "#BUSY_FLAT_RESULTS 1\rlobby\rcafe##");
}
