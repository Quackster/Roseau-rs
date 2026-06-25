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
#[path = "room_command_outcome_tests.rs"]
mod tests;
