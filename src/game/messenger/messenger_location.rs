use crate::game::room::settings::RoomType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessengerLocation {
    HotelView,
    PrivateRoom,
    PublicRoom(String),
}

impl MessengerLocation {
    pub fn from_room(room_type: Option<RoomType>, room_name: Option<&str>) -> Self {
        match room_type {
            Some(RoomType::Private) => Self::PrivateRoom,
            Some(RoomType::Public) => Self::PublicRoom(room_name.unwrap_or_default().to_owned()),
            None => Self::HotelView,
        }
    }

    pub fn display_text(&self) -> String {
        match self {
            Self::HotelView => "On Hotel View".to_owned(),
            Self::PrivateRoom => "In a user flat".to_owned(),
            Self::PublicRoom(name) => name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_java_messenger_locations() {
        assert_eq!(MessengerLocation::HotelView.display_text(), "On Hotel View");
        assert_eq!(
            MessengerLocation::PrivateRoom.display_text(),
            "In a user flat"
        );
        assert_eq!(
            MessengerLocation::PublicRoom("Cafe".to_owned()).display_text(),
            "Cafe"
        );
    }
}
