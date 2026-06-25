use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomState {
    Open,
    Doorbell,
    Password,
}

impl RoomState {
    pub fn state_code(self) -> i32 {
        match self {
            Self::Open => 0,
            Self::Doorbell => 1,
            Self::Password => 2,
        }
    }

    pub fn from_code(state_code: i32) -> Self {
        match state_code {
            1 => Self::Doorbell,
            2 => Self::Password,
            _ => Self::Open,
        }
    }
}

impl Display for RoomState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Open => "open",
            Self::Doorbell => "closed",
            Self::Password => "password",
        };

        f.write_str(value)
    }
}

#[cfg(test)]
#[path = "room_state_tests.rs"]
mod tests;
