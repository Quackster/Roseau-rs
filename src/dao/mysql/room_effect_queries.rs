use crate::dao::mysql::{RoomQueries, SqlExecutionPlan};
use crate::game::room::RoomEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomEffectQueries;

impl RoomEffectQueries {
    pub fn plans(effect: &RoomEffect) -> Vec<SqlExecutionPlan> {
        match effect {
            RoomEffect::SaveRights { room_id, rights } => Self::save_rights_plans(*room_id, rights),
            RoomEffect::StartPublicServer { .. }
            | RoomEffect::ScheduleWalkTicks
            | RoomEffect::ScheduleEventTicks
            | RoomEffect::LoadPassiveObjects { .. }
            | RoomEffect::LoadBots { .. }
            | RoomEffect::RegenerateCollisionMaps
            | RoomEffect::RegisterEvent { .. }
            | RoomEffect::SendDoorbell { .. }
            | RoomEffect::SendOwnerPrivileges { .. }
            | RoomEffect::SendControllerPrivileges { .. }
            | RoomEffect::SendNoControllerPrivileges { .. }
            | RoomEffect::SetRoomUserStatus { .. }
            | RoomEffect::RemoveRoomUserStatus { .. }
            | RoomEffect::MarkRoomUserForUpdate { .. }
            | RoomEffect::LetUserIn { .. }
            | RoomEffect::LeaveRoom { .. }
            | RoomEffect::KickUser { .. }
            | RoomEffect::ClearRuntimeData
            | RoomEffect::RemoveLoadedRoom { .. } => Vec::new(),
        }
    }

    pub fn save_rights_plans(room_id: i32, rights: &[i32]) -> Vec<SqlExecutionPlan> {
        let mut plans = Vec::with_capacity(rights.len() + 1);
        plans.push(RoomQueries::delete_room_rights(room_id).execute_plan());
        plans.extend(
            rights
                .iter()
                .map(|user_id| RoomQueries::insert_room_right(room_id, *user_id).execute_plan()),
        );
        plans
    }
}
