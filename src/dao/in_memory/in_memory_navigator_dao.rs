use crate::dao::{DaoError, NavigatorDao};
use crate::game::room::settings::RoomType;
use crate::game::room::RoomData;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InMemoryNavigatorDao {
    rooms: Vec<RoomData>,
}

impl InMemoryNavigatorDao {
    pub fn new(rooms: impl IntoIterator<Item = RoomData>) -> Self {
        Self {
            rooms: rooms.into_iter().collect(),
        }
    }

    pub fn rooms(&self) -> &[RoomData] {
        &self.rooms
    }
}

impl NavigatorDao for InMemoryNavigatorDao {
    fn rooms_by_like_name(&self, name: &str) -> Result<Vec<RoomData>, DaoError> {
        let needle = name.to_ascii_lowercase();
        Ok(self
            .rooms
            .iter()
            .filter(|room| {
                room.room_type() == RoomType::Private
                    && room.name().to_ascii_lowercase().contains(&needle)
            })
            .cloned()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn room(id: i32, room_type: RoomType, name: &str) -> RoomData {
        RoomData::new(
            id, false, room_type, 7, "alice", name, 0, "", 25, "desc", "model", "class", "wall",
            "floor", false, true,
        )
    }

    #[test]
    fn searches_private_rooms_by_case_insensitive_name_part() {
        let dao = InMemoryNavigatorDao::new([
            room(1, RoomType::Private, "Cafe"),
            room(2, RoomType::Private, "Library"),
            room(3, RoomType::Public, "Cafe Public"),
        ]);

        let rooms = dao.rooms_by_like_name("caf").unwrap();

        assert_eq!(rooms.len(), 1);
        assert_eq!(rooms[0].id(), 1);
    }
}
