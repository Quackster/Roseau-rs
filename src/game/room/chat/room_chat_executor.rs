use crate::dao::{DaoError, RoomChatlog, RoomDao};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomChatExecutor;

impl RoomChatExecutor {
    pub fn save_talk(
        room_dao: &dyn RoomDao,
        username: &str,
        room_id: i32,
        mode: &str,
        message: &str,
    ) -> Result<Option<RoomChatExecution>, DaoError> {
        if !matches!(mode, "CHAT" | "SHOUT") {
            return Ok(None);
        }

        let chatlog = RoomChatlog::new(username, room_id, mode, message);
        room_dao.save_chatlog(&chatlog)?;
        Ok(Some(RoomChatExecution::Saved(chatlog)))
    }

    pub fn save_delivered_whisper(
        room_dao: &dyn RoomDao,
        sender_username: &str,
        sender_id: i32,
        target_username: &str,
        target_id: i32,
        room_id: i32,
        message: &str,
    ) -> Result<Option<RoomChatExecution>, DaoError> {
        if sender_id == target_id {
            return Ok(None);
        }

        let chatlog = RoomChatlog::new(
            sender_username,
            room_id,
            "WHISPER",
            format!("(to: {target_username}) {message}"),
        );
        room_dao.save_chatlog(&chatlog)?;
        Ok(Some(RoomChatExecution::Saved(chatlog)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomChatExecution {
    Saved(RoomChatlog),
}
