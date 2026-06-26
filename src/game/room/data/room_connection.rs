use crate::game::room::model::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct RoomConnection {
    room_id: i32,
    to_id: i32,
    source: Option<Position>,
    door: Position,
}

impl RoomConnection {
    pub fn new(room_id: i32, to_id: i32, door: Position) -> Self {
        Self {
            room_id,
            to_id,
            source: None,
            door,
        }
    }

    pub fn with_source(room_id: i32, to_id: i32, source: Position, door: Position) -> Self {
        Self {
            room_id,
            to_id,
            source: Some(source),
            door,
        }
    }

    pub fn room_id(&self) -> i32 {
        self.room_id
    }

    pub fn set_room_id(&mut self, room_id: i32) {
        self.room_id = room_id;
    }

    pub fn to_id(&self) -> i32 {
        self.to_id
    }

    pub fn set_to_id(&mut self, to_id: i32) {
        self.to_id = to_id;
    }

    pub fn door_position(&self) -> Position {
        self.door
    }

    pub fn source_position(&self) -> Option<Position> {
        self.source
    }

    pub fn matches_source(&self, x: i32, y: i32) -> bool {
        self.source
            .is_some_and(|source| source.x() == x && source.y() == y)
    }
}

#[cfg(test)]
#[path = "room_connection_tests.rs"]
mod tests;
