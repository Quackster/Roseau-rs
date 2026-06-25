use crate::game::entity::EntityType;
use crate::game::player::PlayerDetails;
use crate::game::room::entity::RoomUser;

pub trait Entity {
    fn details(&self) -> &PlayerDetails;
    fn room_user(&self) -> Option<&RoomUser>;
    fn entity_type(&self) -> EntityType;
}

#[cfg(test)]
#[path = "entity_tests.rs"]
mod tests;
