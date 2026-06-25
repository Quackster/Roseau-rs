use crate::dao::mysql::entity::{MessengerFriendshipRow, MessengerMessageRow, MessengerRequestRow};
use crate::dao::mysql::{SqlParameter, SqlQuery};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessengerQueries;

impl MessengerQueries {
    pub fn friends(user_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM messenger_friendships WHERE sender = ? OR receiver = ?",
            [
                SqlParameter::Integer(user_id),
                SqlParameter::Integer(user_id),
            ],
        )
    }

    pub fn requests(user_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM messenger_requests WHERE to_id = ?",
            [SqlParameter::Integer(user_id)],
        )
    }

    pub fn request_exists(from_id: i32, to_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM messenger_requests WHERE (to_id = ? AND from_id = ?) OR (from_id = ? AND to_id = ?) LIMIT 1",
            [
                SqlParameter::Integer(to_id),
                SqlParameter::Integer(from_id),
                SqlParameter::Integer(to_id),
                SqlParameter::Integer(from_id),
            ],
        )
    }

    pub fn create_request(from_id: i32, to_id: i32) -> SqlQuery {
        SqlQuery::new(
            "INSERT INTO messenger_requests (from_id, to_id) VALUES (?, ?)",
            [SqlParameter::Integer(from_id), SqlParameter::Integer(to_id)],
        )
    }

    pub fn remove_request(from_id: i32, to_id: i32) -> SqlQuery {
        SqlQuery::new(
            "DELETE FROM messenger_requests WHERE from_id = ? AND to_id = ?",
            [SqlParameter::Integer(from_id), SqlParameter::Integer(to_id)],
        )
    }

    pub fn create_friend(sender: i32, receiver: i32) -> SqlQuery {
        SqlQuery::new(
            "INSERT INTO messenger_friendships (sender, receiver) VALUES (?, ?)",
            [
                SqlParameter::Integer(sender),
                SqlParameter::Integer(receiver),
            ],
        )
    }

    pub fn remove_friend(friend_id: i32, user_id: i32) -> SqlQuery {
        SqlQuery::new(
            "DELETE FROM messenger_friendships WHERE (sender = ? AND receiver = ?) OR (receiver = ? AND sender = ?)",
            [
                SqlParameter::Integer(user_id),
                SqlParameter::Integer(friend_id),
                SqlParameter::Integer(user_id),
                SqlParameter::Integer(friend_id),
            ],
        )
    }

    pub fn create_message(from_id: i32, to_id: i32, time_sent: i64, message: &str) -> SqlQuery {
        SqlQuery::new(
            "INSERT INTO messenger_messages (from_id, to_id, time_sent, message, unread) VALUES (?, ?, ?, ?, ?)",
            [
                SqlParameter::Integer(from_id),
                SqlParameter::Integer(to_id),
                SqlParameter::Long(time_sent),
                SqlParameter::Text(message.to_owned()),
                SqlParameter::Bool(true),
            ],
        )
    }

    pub fn unread_messages(user_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM messenger_messages WHERE to_id = ? AND unread = ?",
            [SqlParameter::Integer(user_id), SqlParameter::Bool(true)],
        )
    }

    pub fn mark_message_read(message_id: i32) -> SqlQuery {
        SqlQuery::new(
            "UPDATE messenger_messages SET unread = ? WHERE id = ?",
            [SqlParameter::Bool(false), SqlParameter::Integer(message_id)],
        )
    }

    pub fn friendship_table() -> &'static str {
        MessengerFriendshipRow::TABLE
    }

    pub fn message_table() -> &'static str {
        MessengerMessageRow::TABLE
    }

    pub fn request_table() -> &'static str {
        MessengerRequestRow::TABLE
    }
}
