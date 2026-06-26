use crate::dao::mysql::{RoomQueries, SqlExecutionPlan};
use crate::dao::{CreateRoom, RoomChatlog};
use crate::game::player::PlayerDetails;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomCommandQueries;

impl RoomCommandQueries {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        owner: &PlayerDetails,
    ) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::CreateFlat {
                room_name,
                room_model,
                state,
                show_owner_name,
                ..
            } => Some(
                RoomQueries::create_room(&CreateRoom::new(
                    owner,
                    room_name,
                    "",
                    room_model,
                    *state,
                    *show_owner_name,
                ))
                .insert_returning_id_plan(),
            ),
            IncomingExecutionEffect::DeleteFlat { room_id } => {
                Some(RoomQueries::delete_room(*room_id).execute_plan())
            }
            IncomingExecutionEffect::UpdateFlat {
                room_id,
                room_name,
                state,
                show_owner_name,
            } => Some(
                RoomQueries::update_flat(*room_id, room_name, *state, *show_owner_name)
                    .execute_plan(),
            ),
            IncomingExecutionEffect::SetFlatInfo {
                room_id,
                description,
                password,
                all_super_user,
            } => Some(
                RoomQueries::update_flat_info(*room_id, description, password, *all_super_user)
                    .execute_plan(),
            ),
            _ => None,
        }
    }

    pub fn apply_decoration_plan(
        effect: &IncomingExecutionEffect,
        room_id: i32,
        data: &str,
    ) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::ApplyDecoration { decoration, .. } => {
                Some(RoomQueries::update_decoration(room_id, decoration, data)?.execute_plan())
            }
            _ => None,
        }
    }

    pub fn read_plan(effect: &IncomingExecutionEffect) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::GetFlatInfo { room_id }
            | IncomingExecutionEffect::TryFlat { room_id, .. } => {
                Some(RoomQueries::room(*room_id).read_plan())
            }
            _ => None,
        }
    }

    pub fn chatlog_plan(
        effect: &IncomingExecutionEffect,
        username: &str,
        room_id: i32,
        now: i64,
    ) -> Option<SqlExecutionPlan> {
        match effect {
            IncomingExecutionEffect::Talk { mode, message }
                if matches!(mode.as_str(), "CHAT" | "SHOUT") =>
            {
                let chatlog = RoomChatlog::new(username, room_id, mode, message);
                Some(RoomQueries::save_chatlog(&chatlog, now).execute_plan())
            }
            _ => None,
        }
    }
}
