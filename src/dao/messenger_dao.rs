use crate::dao::DaoError;
use crate::game::messenger::{MessengerMessage, MessengerUser};

pub trait MessengerDao {
    fn friends(&self, user_id: i32) -> Result<Vec<MessengerUser>, DaoError>;
    fn requests(&self, user_id: i32) -> Result<Vec<MessengerUser>, DaoError>;
    fn new_request(&self, from_id: i32, to_id: i32) -> Result<bool, DaoError>;
    fn remove_request(&self, from_id: i32, to_id: i32) -> Result<(), DaoError>;
    fn new_friend(&self, sender: i32, receiver: i32) -> Result<bool, DaoError>;
    fn remove_friend(&self, friend_id: i32, user_id: i32) -> Result<(), DaoError>;
    fn request_exists(&self, from_id: i32, to_id: i32) -> Result<bool, DaoError>;
    fn new_message(&self, from_id: i32, to_id: i32, message: &str) -> Result<i32, DaoError>;
    fn unread_messages(&self, user_id: i32) -> Result<Vec<MessengerMessage>, DaoError>;
    fn mark_message_read(&self, message_id: i32) -> Result<(), DaoError>;
}
