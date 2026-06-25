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
mod tests {
    use super::*;

    #[test]
    fn maps_java_room_type_codes() {
        assert_eq!(RoomType::from_code(1), RoomType::Public);
        assert_eq!(RoomType::from_code(0), RoomType::Private);
        assert_eq!(RoomType::from_code(99), RoomType::Private);
        assert_eq!(RoomType::Public.type_code(), 1);
        assert_eq!(RoomType::Private.type_code(), 0);
    }
}
