#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomType {
    Public,
    Private,
}

impl RoomType {
    pub fn type_code(self) -> i32 {
        match self {
            Self::Public => 1,
            Self::Private => 0,
        }
    }

    pub fn from_code(type_code: i32) -> Self {
        match type_code {
            1 => Self::Public,
            _ => Self::Private,
        }
    }
}

#[cfg(test)]
#[path = "room_type_tests.rs"]
mod tests;
