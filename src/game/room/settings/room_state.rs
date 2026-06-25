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
mod tests {
    use super::*;

    #[test]
    fn maps_java_room_state_codes() {
        assert_eq!(RoomState::from_code(0), RoomState::Open);
        assert_eq!(RoomState::from_code(1), RoomState::Doorbell);
        assert_eq!(RoomState::from_code(2), RoomState::Password);
        assert_eq!(RoomState::from_code(99), RoomState::Open);
    }

    #[test]
    fn renders_java_room_state_strings() {
        assert_eq!(RoomState::Open.to_string(), "open");
        assert_eq!(RoomState::Doorbell.to_string(), "closed");
        assert_eq!(RoomState::Password.to_string(), "password");
    }
}
