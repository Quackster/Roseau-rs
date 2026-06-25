use crate::game::entity::{Entity, EntityType};
use crate::game::player::PlayerDetails;
use crate::game::room::entity::RoomUser;
use crate::game::room::model::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct Bot {
    details: PlayerDetails,
    start_position: Position,
    positions: Vec<(i32, i32)>,
    responses: Vec<String>,
    triggers: Vec<String>,
}

impl Bot {
    pub fn new(
        start_position: Position,
        positions: Vec<(i32, i32)>,
        responses: Vec<String>,
        triggers: Vec<String>,
    ) -> Self {
        Self {
            details: PlayerDetails::new(),
            start_position,
            positions,
            responses,
            triggers,
        }
    }

    pub fn contains_trigger(&self, phrase: &str) -> Option<&str> {
        let phrase = phrase.to_lowercase();

        self.triggers
            .iter()
            .find(|trigger| phrase.contains(&trigger.to_lowercase()))
            .map(String::as_str)
    }

    pub fn response_at(&self, index: usize, username: &str, item: &str) -> Option<String> {
        let response = self.responses.get(index)?;

        Some(
            response
                .replace("%username%", username)
                .replace("%item%", item),
        )
    }

    pub fn first_response(&self, username: &str, item: &str) -> Option<String> {
        self.response_at(0, username, item)
    }

    pub fn details(&self) -> &PlayerDetails {
        &self.details
    }

    pub fn details_mut(&mut self) -> &mut PlayerDetails {
        &mut self.details
    }

    pub fn entity_type(&self) -> EntityType {
        EntityType::Bot
    }

    pub fn start_position(&self) -> Position {
        self.start_position
    }

    pub fn positions(&self) -> &[(i32, i32)] {
        &self.positions
    }

    pub fn responses(&self) -> &[String] {
        &self.responses
    }

    pub fn triggers(&self) -> &[String] {
        &self.triggers
    }
}

impl Entity for Bot {
    fn details(&self) -> &PlayerDetails {
        self.details()
    }

    fn room_user(&self) -> Option<&RoomUser> {
        None
    }

    fn entity_type(&self) -> EntityType {
        self.entity_type()
    }
}
