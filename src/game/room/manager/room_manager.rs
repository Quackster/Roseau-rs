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

    pub fn get_room_by_id_mut(&mut self, room_id: i32) -> Option<&mut RoomSummary> {
        self.loaded_rooms.get_mut(&room_id)
    }

    pub fn get_room_by_port(&self, port: i32, base_port: i32) -> Option<&RoomSummary> {
        self.loaded_rooms
            .values()
            .find(|room| room.data().server_port(base_port) == port)
    }

    pub fn get_room_by_port_mut(&mut self, port: i32, base_port: i32) -> Option<&mut RoomSummary> {
        self.loaded_rooms
            .values_mut()
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
#[path = "room_manager_tests.rs"]
mod tests;
