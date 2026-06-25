#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomEvent {
    ticked: i64,
}

impl RoomEvent {
    pub fn new() -> Self {
        Self { ticked: 0 }
    }

    pub fn increase_ticked(&mut self) {
        self.ticked += 1;
    }

    pub fn can_tick(&self, second_interval: i64) -> bool {
        second_interval != 0 && self.ticked % second_interval == 0
    }

    pub fn ticked(&self) -> i64 {
        self.ticked
    }
}

impl Default for RoomEvent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_java_room_event_tick_counter() {
        let mut event = RoomEvent::new();

        assert!(event.can_tick(2));
        event.increase_ticked();
        assert!(!event.can_tick(2));
        event.increase_ticked();
        assert!(event.can_tick(2));
        assert_eq!(event.ticked(), 2);
    }
}
