use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessengerFriendshipRow {
    pub id: i32,
    pub sender: i32,
    pub receiver: i32,
}

impl MessengerFriendshipRow {
    pub const TABLE: &'static str = "messenger_friendships";

    pub const fn new(id: i32, sender: i32, receiver: i32) -> Self {
        Self {
            id,
            sender,
            receiver,
        }
    }
}

impl TryFrom<&SqlRow> for MessengerFriendshipRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_i32("sender")?,
            row.required_i32("receiver")?,
        ))
    }
}
