use crate::game::room::model::Position;
use crate::game::room::{RoomMapping, RoomOccupant};

impl RoomMapping {
    pub fn nearby_occupants(
        &self,
        entity_id: i32,
        start: Position,
        distance: i32,
        occupants: &[RoomOccupant],
    ) -> Vec<RoomOccupant> {
        occupants
            .iter()
            .copied()
            .filter(|occupant| {
                occupant.entity_id() != entity_id && start.distance(occupant.position()) <= distance
            })
            .collect()
    }

    pub fn occupant_at(&self, x: i32, y: i32, occupants: &[RoomOccupant]) -> Option<RoomOccupant> {
        let position = Position::new(x, y, 0.0);
        occupants
            .iter()
            .copied()
            .find(|occupant| occupant.position().is_match(position))
    }

    pub fn occupant_goal_at(
        &self,
        x: i32,
        y: i32,
        occupants: &[RoomOccupant],
    ) -> Option<RoomOccupant> {
        let position = Position::new(x, y, 0.0);
        occupants
            .iter()
            .copied()
            .find(|occupant| occupant.goal().is_some_and(|goal| goal.is_match(position)))
    }
}
