use crate::dao::DaoError;
use crate::game::player::{Bot, PlayerDetails};
use crate::game::room::model::RoomModel;
use crate::game::room::{RoomConnection, RoomData};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateRoom {
    pub owner_id: i32,
    pub owner_name: String,
    pub name: String,
    pub description: String,
    pub model: String,
    pub state: i32,
    pub show_owner_name: bool,
}

impl CreateRoom {
    pub fn new(
        owner: &PlayerDetails,
        name: impl Into<String>,
        description: impl Into<String>,
        model: impl Into<String>,
        state: i32,
        show_owner_name: bool,
    ) -> Self {
        Self {
            owner_id: owner.id(),
            owner_name: owner.username().to_owned(),
            name: name.into(),
            description: description.into(),
            model: model.into(),
            state,
            show_owner_name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomChatlog {
    pub username: String,
    pub room_id: i32,
    pub chat_type: String,
    pub message: String,
}

impl RoomChatlog {
    pub fn new(
        username: impl Into<String>,
        room_id: i32,
        chat_type: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            username: username.into(),
            room_id,
            chat_type: chat_type.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicRoomDescriptor {
    id: i32,
    name: String,
}

impl PublicRoomDescriptor {
    pub fn new(id: i32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub trait RoomDao {
    fn public_rooms(&self, store_in_memory: bool) -> Result<Vec<RoomData>, DaoError>;
    fn player_rooms(
        &self,
        details: &PlayerDetails,
        store_in_memory: bool,
    ) -> Result<Vec<RoomData>, DaoError>;
    fn room(&self, room_id: i32, store_in_memory: bool) -> Result<Option<RoomData>, DaoError>;
    fn room_rights(&self, room_id: i32) -> Result<Vec<i32>, DaoError>;
    fn update_room(&self, room: &RoomData) -> Result<(), DaoError>;
    fn model(&self, model: &str) -> Result<Option<RoomModel>, DaoError>;
    fn delete_room(&self, room: &RoomData) -> Result<(), DaoError>;
    fn create_room(&self, room: &CreateRoom) -> Result<RoomData, DaoError>;
    fn room_connections(&self, room_id: i32) -> Result<Vec<RoomConnection>, DaoError>;
    fn bots(&self, room_id: i32) -> Result<Vec<Bot>, DaoError>;
    fn save_room_rights(&self, room_id: i32, rights: &[i32]) -> Result<(), DaoError>;
    fn save_chatlog(&self, chatlog: &RoomChatlog) -> Result<(), DaoError>;
    fn public_room_descriptors(&self) -> Result<Vec<PublicRoomDescriptor>, DaoError>;
    fn public_room_ids(&self) -> Result<Vec<i32>, DaoError> {
        Ok(self
            .public_room_descriptors()?
            .into_iter()
            .map(|room| room.id())
            .collect())
    }
    fn latest_player_rooms(
        &self,
        blacklist: &[i32],
        multiplier: i32,
    ) -> Result<Vec<RoomData>, DaoError>;
}

#[cfg(test)]
#[path = "room_dao_tests.rs"]
mod tests;
