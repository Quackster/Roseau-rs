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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::in_memory::InMemoryRoomDao;

    #[test]
    fn saves_chat_and_shout_logs() {
        let dao = InMemoryRoomDao::new();

        let chat = RoomChatExecutor::save_talk(&dao, "alice", 42, "CHAT", "hello").unwrap();
        let shout = RoomChatExecutor::save_talk(&dao, "alice", 42, "SHOUT", "hey").unwrap();

        assert_eq!(
            chat,
            Some(RoomChatExecution::Saved(RoomChatlog::new(
                "alice", 42, "CHAT", "hello"
            )))
        );
        assert_eq!(
            shout,
            Some(RoomChatExecution::Saved(RoomChatlog::new(
                "alice", 42, "SHOUT", "hey"
            )))
        );
        assert_eq!(dao.chatlogs().len(), 2);
    }

    #[test]
    fn ignores_non_persisted_talk_modes() {
        let dao = InMemoryRoomDao::new();

        let result = RoomChatExecutor::save_talk(&dao, "alice", 42, "WHISPER", "hello").unwrap();

        assert_eq!(result, None);
        assert!(dao.chatlogs().is_empty());
    }

    #[test]
    fn saves_delivered_whisper_with_java_target_prefix() {
        let dao = InMemoryRoomDao::new();

        let result =
            RoomChatExecutor::save_delivered_whisper(&dao, "alice", 1, "bob", 2, 42, "psst")
                .unwrap();

        assert_eq!(
            result,
            Some(RoomChatExecution::Saved(RoomChatlog::new(
                "alice",
                42,
                "WHISPER",
                "(to: bob) psst",
            )))
        );
        assert_eq!(dao.chatlogs()[0].message, "(to: bob) psst");
    }

    #[test]
    fn skips_self_whisper_persistence() {
        let dao = InMemoryRoomDao::new();

        let result =
            RoomChatExecutor::save_delivered_whisper(&dao, "alice", 1, "alice", 1, 42, "psst")
                .unwrap();

        assert_eq!(result, None);
        assert!(dao.chatlogs().is_empty());
    }
}
