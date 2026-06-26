use crate::game::room::RoomSummary;
use crate::messages::outgoing::{AllUnits, PublicUnit, UnitMembers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomUnitOutcome {
    public_rooms: Vec<RoomSummary>,
    member_names: Option<Vec<String>>,
    send_empty_all_units: bool,
}

impl RoomUnitOutcome {
    pub fn listener(public_rooms: impl IntoIterator<Item = RoomSummary>) -> Self {
        Self {
            public_rooms: public_rooms.into_iter().collect(),
            member_names: None,
            send_empty_all_units: true,
        }
    }

    pub fn unit_members(
        public_rooms: impl IntoIterator<Item = RoomSummary>,
        member_names: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            public_rooms: public_rooms.into_iter().collect(),
            member_names: Some(member_names.into_iter().map(Into::into).collect()),
            send_empty_all_units: false,
        }
    }

    pub fn missing_room() -> Self {
        Self {
            public_rooms: Vec::new(),
            member_names: None,
            send_empty_all_units: false,
        }
    }

    pub fn all_units(&self, server_ip: impl Into<String>, base_port: i32) -> Option<AllUnits> {
        if self.public_rooms.is_empty() && !self.send_empty_all_units {
            return None;
        }

        Some(AllUnits::new(
            server_ip,
            self.public_rooms
                .iter()
                .map(|room| public_unit(room, base_port)),
        ))
    }

    pub fn unit_members_packet(&self) -> Option<UnitMembers> {
        self.member_names
            .as_ref()
            .map(|names| UnitMembers::new(names.clone()))
    }
}

fn public_unit(room: &RoomSummary, base_port: i32) -> PublicUnit {
    let data = room.data();
    let users_now = users_now(room.player_count());
    PublicUnit::new(
        data.name(),
        users_now,
        data.users_max(),
        data.server_port(base_port),
        data.class_name(),
        data.model_name(),
    )
}

fn users_now(player_count: usize) -> i32 {
    i32::try_from(player_count).unwrap_or(i32::MAX)
}

#[cfg(test)]
#[path = "room_unit_outcome_tests.rs"]
mod tests;
