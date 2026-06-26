use crate::dao::mysql::entity::{
    RoomBotRow, RoomChatlogRow, RoomModelRow, RoomPublicConnectionRow, RoomRightRow, RoomRow,
};
use crate::dao::mysql::{SqlParameter, SqlQuery};
use crate::dao::{CreateRoom, RoomChatlog};
use crate::game::room::settings::RoomType;
use crate::game::room::RoomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomQueries;

impl RoomQueries {
    pub fn public_rooms() -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM rooms WHERE enabled = ? AND room_type = ? ORDER BY order_id ASC",
            [
                SqlParameter::Bool(true),
                SqlParameter::Integer(RoomType::Public.type_code()),
            ],
        )
    }

    pub fn public_room_descriptors() -> SqlQuery {
        SqlQuery::new(
            "SELECT id, name FROM rooms WHERE enabled = ? AND room_type = ? AND hidden = ? ORDER BY order_id ASC",
            [
                SqlParameter::Bool(true),
                SqlParameter::Integer(RoomType::Public.type_code()),
                SqlParameter::Bool(false),
            ],
        )
    }

    pub fn room_connections(room_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM room_public_connections WHERE room_id = ?",
            [SqlParameter::Integer(room_id)],
        )
    }

    pub fn player_rooms(owner_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM rooms WHERE owner_id = ?",
            [SqlParameter::Integer(owner_id)],
        )
    }

    pub fn latest_player_rooms(range: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM rooms WHERE room_type = ? ORDER BY id DESC LIMIT 11 OFFSET ?",
            [
                SqlParameter::Integer(RoomType::Private.type_code()),
                SqlParameter::Integer(range * 11),
            ],
        )
    }

    pub fn room(room_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM rooms WHERE id = ? LIMIT 1",
            [SqlParameter::Integer(room_id)],
        )
    }

    pub fn room_rights(room_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM room_rights WHERE room_id = ?",
            [SqlParameter::Integer(room_id)],
        )
    }

    pub fn delete_room_rights(room_id: i32) -> SqlQuery {
        SqlQuery::new(
            "DELETE FROM room_rights WHERE room_id = ?",
            [SqlParameter::Integer(room_id)],
        )
    }

    pub fn insert_room_right(room_id: i32, user_id: i32) -> SqlQuery {
        SqlQuery::new(
            "INSERT INTO room_rights (room_id, user_id) VALUES (?, ?)",
            [
                SqlParameter::Integer(room_id),
                SqlParameter::Integer(user_id),
            ],
        )
    }

    pub fn delete_room(room_id: i32) -> SqlQuery {
        SqlQuery::new(
            "DELETE FROM rooms WHERE id = ?",
            [SqlParameter::Integer(room_id)],
        )
    }

    pub fn create_room(room: &CreateRoom) -> SqlQuery {
        SqlQuery::new(
            "INSERT INTO rooms (name, description, owner_id, model, state, show_owner_name, room_type) VALUES (?, ?, ?, ?, ?, ?, ?)",
            [
                SqlParameter::Text(room.name.clone()),
                SqlParameter::Text(room.description.clone()),
                SqlParameter::Integer(room.owner_id),
                SqlParameter::Text(room.model.clone()),
                SqlParameter::Integer(room.state),
                SqlParameter::Bool(room.show_owner_name),
                SqlParameter::Integer(RoomType::Private.type_code()),
            ],
        )
    }

    pub fn save_chatlog(chatlog: &RoomChatlog, now: i64) -> SqlQuery {
        SqlQuery::new(
            "INSERT INTO room_chatlogs (user, room_id, timestamp, message_type, message) VALUES (?, ?, ?, ?, ?)",
            [
                SqlParameter::Text(chatlog.username.clone()),
                SqlParameter::Integer(chatlog.room_id),
                SqlParameter::Long(now),
                SqlParameter::Integer(Self::chat_type_code(&chatlog.chat_type)),
                SqlParameter::Text(chatlog.message.clone()),
            ],
        )
    }

    pub fn update_room(room: &RoomData) -> SqlQuery {
        SqlQuery::new(
            "UPDATE rooms SET name = ?, description = ?, state = ?, password = ?, wallpaper = ?, floor = ?, allsuperuser = ?, show_owner_name = ? WHERE id = ?",
            [
                SqlParameter::Text(room.name().to_owned()),
                SqlParameter::Text(room.description().to_owned()),
                SqlParameter::Integer(room.state().state_code()),
                SqlParameter::Text(room.password().to_owned()),
                SqlParameter::Text(room.wall().to_owned()),
                SqlParameter::Text(room.floor().to_owned()),
                SqlParameter::Bool(room.has_all_super_user()),
                SqlParameter::Bool(room.show_owner_name()),
                SqlParameter::Integer(room.id()),
            ],
        )
    }

    pub fn update_flat(
        room_id: i32,
        room_name: &str,
        state: i32,
        show_owner_name: bool,
    ) -> SqlQuery {
        SqlQuery::new(
            "UPDATE rooms SET name = ?, state = ?, show_owner_name = ? WHERE id = ?",
            [
                SqlParameter::Text(room_name.to_owned()),
                SqlParameter::Integer(state),
                SqlParameter::Bool(show_owner_name),
                SqlParameter::Integer(room_id),
            ],
        )
    }

    pub fn update_flat_info(
        room_id: i32,
        description: &str,
        password: &str,
        all_super_user: bool,
    ) -> SqlQuery {
        SqlQuery::new(
            "UPDATE rooms SET description = ?, password = ?, allsuperuser = ? WHERE id = ?",
            [
                SqlParameter::Text(description.to_owned()),
                SqlParameter::Text(password.to_owned()),
                SqlParameter::Bool(all_super_user),
                SqlParameter::Integer(room_id),
            ],
        )
    }

    pub fn update_decoration(
        room_id: i32,
        decoration: &str,
        data: impl Into<String>,
    ) -> Option<SqlQuery> {
        let column = match decoration {
            "wallpaper" => "wallpaper",
            "floor" => "floor",
            _ => return None,
        };

        Some(SqlQuery::new(
            format!("UPDATE rooms SET {column} = ? WHERE id = ?"),
            [
                SqlParameter::Text(data.into()),
                SqlParameter::Integer(room_id),
            ],
        ))
    }

    pub fn bots(room_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM room_bots WHERE room_id = ?",
            [SqlParameter::Integer(room_id)],
        )
    }

    pub fn models() -> SqlQuery {
        SqlQuery::select_all(RoomModelRow::TABLE)
    }

    pub fn chat_type_code(chat_type: &str) -> i32 {
        match chat_type {
            "CHAT" => 0,
            "SHOUT" => 1,
            _ => 2,
        }
    }

    pub fn room_table() -> &'static str {
        RoomRow::TABLE
    }

    pub fn right_table() -> &'static str {
        RoomRightRow::TABLE
    }

    pub fn connection_table() -> &'static str {
        RoomPublicConnectionRow::TABLE
    }

    pub fn bot_table() -> &'static str {
        RoomBotRow::TABLE
    }

    pub fn chatlog_table() -> &'static str {
        RoomChatlogRow::TABLE
    }
}
