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
