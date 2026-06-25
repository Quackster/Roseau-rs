use crate::game::room::model::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct RoomConnection {
    room_id: i32,
    to_id: i32,
    door: Position,
}

impl RoomConnection {
    pub fn new(room_id: i32, to_id: i32, door: Position) -> Self {
        Self {
            room_id,
            to_id,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_room_connection_fields() {
        let mut connection = RoomConnection::new(1, 2, Position::new(3, 4, 0.0));

        assert_eq!(connection.room_id(), 1);
        assert_eq!(connection.to_id(), 2);
        assert_eq!(connection.door_position(), Position::new(3, 4, 0.0));

        connection.set_room_id(5);
        connection.set_to_id(6);

        assert_eq!(connection.room_id(), 5);
        assert_eq!(connection.to_id(), 6);
    }
}
