use std::collections::HashMap;

use crate::game::room::settings::RoomType;
use crate::game::room::RoomSummary;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RoomManager {
    loaded_rooms: HashMap<i32, RoomSummary>,
}

impl RoomManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, room: RoomSummary) -> bool {
        let room_id = room.data().id();
        if self.loaded_rooms.contains_key(&room_id) {
            return false;
        }

        self.loaded_rooms.insert(room_id, room);
        true
    }

    pub fn remove_loaded_room(&mut self, room_id: i32) -> Option<RoomSummary> {
        self.loaded_rooms.remove(&room_id)
    }

    pub fn get_public_rooms(&self) -> Vec<&RoomSummary> {
        let mut rooms: Vec<_> = self
            .loaded_rooms
            .values()
            .filter(|room| room.data().room_type() == RoomType::Public && !room.data().is_hidden())
            .collect();
        rooms.sort_by_key(|room| room.order_id());
        rooms
    }

    pub fn get_popular_rooms(&self, multiplier: usize) -> Vec<&RoomSummary> {
        let range = if multiplier > 0 { multiplier / 11 } else { 0 };
        let mut rooms: Vec<_> = self
            .loaded_rooms
            .values()
            .filter(|room| {
                room.data().room_type() == RoomType::Private
                    && !room.data().is_hidden()
                    && room.player_count() > 0
            })
            .collect();
        rooms.sort_by(|left, right| right.player_count().cmp(&left.player_count()));
        rooms.into_iter().skip(range).collect()
    }

    pub fn get_player_rooms(&self, user_id: i32) -> Vec<&RoomSummary> {
        let mut rooms: Vec<_> = self
            .loaded_rooms
            .values()
            .filter(|room| room.data().owner_id() == user_id && !room.data().is_hidden())
            .collect();
        rooms.sort_by_key(|room| room.data().id());
        rooms
    }

    pub fn get_room_by_id(&self, room_id: i32) -> Option<&RoomSummary> {
        self.loaded_rooms.get(&room_id)
    }

    pub fn get_room_by_port(&self, port: i32, base_port: i32) -> Option<&RoomSummary> {
        self.loaded_rooms
            .values()
            .find(|room| room.data().server_port(base_port) == port)
    }

    pub fn get_room_by_name(&self, name: &str) -> Option<&RoomSummary> {
        self.loaded_rooms
            .values()
            .find(|room| room.data().name() == name)
    }

    pub fn loaded_rooms(&self) -> &HashMap<i32, RoomSummary> {
        &self.loaded_rooms
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::room::settings::RoomType;
    use crate::game::room::RoomData;

    fn room(id: i32, room_type: RoomType, owner_id: i32, hidden: bool) -> RoomSummary {
        let data = RoomData::new(
            id,
            hidden,
            room_type,
            owner_id,
            "owner",
            format!("room{id}"),
            0,
            "",
            25,
            "desc",
            "model",
            "class",
            "wall",
            "floor",
            false,
            true,
        );
        RoomSummary::new(data)
    }

    #[test]
    fn adds_once_and_finds_loaded_rooms() {
        let mut manager = RoomManager::new();
        let mut public = room(1, RoomType::Public, 7, false);
        public.set_order_id(2);

        assert!(manager.add(public));
        assert!(!manager.add(room(1, RoomType::Public, 7, false)));

        assert_eq!(manager.get_room_by_id(1).unwrap().data().name(), "room1");
        assert_eq!(
            manager.get_room_by_port(37121, 37120).unwrap().data().id(),
            1
        );
        assert_eq!(manager.get_room_by_name("room1").unwrap().data().id(), 1);

        assert_eq!(manager.remove_loaded_room(1).unwrap().data().id(), 1);
        assert!(manager.get_room_by_id(1).is_none());
    }

    #[test]
    fn lists_public_popular_and_owner_rooms_like_java_manager() {
        let mut manager = RoomManager::new();
        let mut public_late = room(1, RoomType::Public, 7, false);
        public_late.set_order_id(2);
        let mut public_first = room(2, RoomType::Public, 8, false);
        public_first.set_order_id(1);
        let mut private_busy = room(3, RoomType::Private, 7, false);
        private_busy.set_player_count(4);
        let mut private_quiet = room(4, RoomType::Private, 7, false);
        private_quiet.set_player_count(1);

        manager.add(public_late);
        manager.add(public_first);
        manager.add(private_quiet);
        manager.add(private_busy);
        manager.add(room(5, RoomType::Public, 7, true));

        let public_ids: Vec<_> = manager
            .get_public_rooms()
            .into_iter()
            .map(|room| room.data().id())
            .collect();
        let popular_ids: Vec<_> = manager
            .get_popular_rooms(0)
            .into_iter()
            .map(|room| room.data().id())
            .collect();
        let owner_ids: Vec<_> = manager
            .get_player_rooms(7)
            .into_iter()
            .map(|room| room.data().id())
            .collect();

        assert_eq!(public_ids, vec![2, 1]);
        assert_eq!(popular_ids, vec![3, 4]);
        assert_eq!(owner_ids, vec![1, 3, 4]);
    }
}
