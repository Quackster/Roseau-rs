use crate::dao::mysql::{RoomQueries, SqlExecutionPlan};
use crate::dao::RoomChatlog;
use crate::game::player::PlayerManager;
use crate::game::room::entity::RoomUserEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomUserEffectQueries;

impl RoomUserEffectQueries {
    pub fn delivered_whisper_chatlog_plan(
        effect: &RoomUserEffect,
        sender_id: i32,
        room_id: i32,
        now: i64,
        player_manager: &PlayerManager,
    ) -> Option<SqlExecutionPlan> {
        let RoomUserEffect::Whisper {
            username,
            target_username: Some(target_username),
            message,
        } = effect
        else {
            return None;
        };

        let target = player_manager.get_by_name(target_username)?;
        if target.details().id() == sender_id {
            return None;
        }

        let chatlog = RoomChatlog::new(
            username,
            room_id,
            "WHISPER",
            format!("(to: {target_username}) {message}"),
        );
        Some(RoomQueries::save_chatlog(&chatlog, now).execute_plan())
    }
}
