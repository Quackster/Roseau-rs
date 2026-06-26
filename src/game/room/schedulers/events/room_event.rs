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
#[path = "room_event_tests.rs"]
mod tests;
