use crate::game::room::RoomData;
use crate::messages::outgoing::{FlatCreated, FlatInfo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomCommandOutcome {
    Created(RoomData),
    FlatInfo(RoomData),
    Ignored,
}

impl RoomCommandOutcome {
    pub fn created(room: &RoomData) -> Self {
        Self::Created(room.clone())
    }

    pub fn flat_info(room: &RoomData) -> Self {
        Self::FlatInfo(room.clone())
    }

    pub fn flat_created(&self) -> Option<FlatCreated> {
        match self {
            Self::Created(room) => Some(FlatCreated::new(room.id(), room.name())),
            Self::FlatInfo(_) | Self::Ignored => None,
        }
    }

    pub fn flat_info_packet(&self) -> Option<FlatInfo> {
        match self {
            Self::FlatInfo(room) => Some(FlatInfo::new(room.id())),
            Self::Created(_) | Self::Ignored => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::room::settings::RoomType;
    use crate::messages::OutgoingMessage;

    fn room_data() -> RoomData {
        RoomData::new(
            42,
            false,
            RoomType::Private,
            7,
            "alice",
            "Tea Room",
            0,
            "",
            25,
            "description",
            "model_a",
            "class",
            "wall",
            "floor",
            false,
            true,
        )
    }

    #[test]
    fn maps_created_room_to_flat_created_packet() {
        let outcome = RoomCommandOutcome::created(&room_data());
        let mut response = outcome.flat_created().unwrap().compose();

        assert_eq!(response.get(), "#FLATCREATED\r42 Tea Room##");
        assert!(outcome.flat_info_packet().is_none());
    }

    #[test]
    fn maps_loaded_room_to_flat_info_packet() {
        let outcome = RoomCommandOutcome::flat_info(&room_data());
        let mut response = outcome.flat_info_packet().unwrap().compose();

        assert_eq!(response.get(), "#SETFLATINFO\r/42/##");
        assert!(outcome.flat_created().is_none());
    }

    #[test]
    fn ignored_room_command_has_no_packet() {
        assert!(RoomCommandOutcome::Ignored.flat_created().is_none());
        assert!(RoomCommandOutcome::Ignored.flat_info_packet().is_none());
    }
}
