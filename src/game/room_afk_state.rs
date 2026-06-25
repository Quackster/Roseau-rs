#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomAfkState {
    user_id: i32,
    seconds_remaining: i64,
}

impl RoomAfkState {
    pub fn new(user_id: i32, seconds_remaining: i64) -> Self {
        Self {
            user_id,
            seconds_remaining,
        }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn seconds_remaining(&self) -> i64 {
        self.seconds_remaining
    }

    pub fn tick(&mut self) {
        if self.seconds_remaining > 0 {
            self.seconds_remaining -= 1;
        }
    }

    pub fn should_kick(&self) -> bool {
        self.seconds_remaining == 0
    }
}
