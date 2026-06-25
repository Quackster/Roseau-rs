use crate::game::room::model::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoomOccupant {
    entity_id: i32,
    position: Position,
    goal: Option<Position>,
}

impl RoomOccupant {
    pub fn new(entity_id: i32, position: Position, goal: Option<Position>) -> Self {
        Self {
            entity_id,
            position,
            goal,
        }
    }

    pub fn entity_id(&self) -> i32 {
        self.entity_id
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn goal(&self) -> Option<Position> {
        self.goal
    }
}
