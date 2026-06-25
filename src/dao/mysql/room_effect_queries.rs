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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionKind, SqlParameter};

    #[test]
    fn maps_save_rights_effect_to_delete_then_insert_plans() {
        let plans = RoomEffectQueries::plans(&RoomEffect::SaveRights {
            room_id: 42,
            rights: vec![7, 8],
        });

        assert_eq!(plans.len(), 3);
        assert_eq!(plans[0].kind(), SqlExecutionKind::Execute);
        assert_eq!(plans[0].sql(), "DELETE FROM room_rights WHERE room_id = ?");
        assert_eq!(plans[0].parameters(), &[SqlParameter::Integer(42)]);
        assert_eq!(
            plans[1].sql(),
            "INSERT INTO room_rights (room_id, user_id) VALUES (?, ?)"
        );
        assert_eq!(
            plans[1].parameters(),
            &[SqlParameter::Integer(42), SqlParameter::Integer(7)]
        );
        assert_eq!(
            plans[2].parameters(),
            &[SqlParameter::Integer(42), SqlParameter::Integer(8)]
        );
    }

    #[test]
    fn keeps_empty_rights_as_delete_only() {
        let plans = RoomEffectQueries::save_rights_plans(42, &[]);

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].sql(), "DELETE FROM room_rights WHERE room_id = ?");
    }

    #[test]
    fn ignores_non_persistent_room_effects() {
        assert!(RoomEffectQueries::plans(&RoomEffect::ScheduleWalkTicks).is_empty());
        assert!(
            RoomEffectQueries::plans(&RoomEffect::SendControllerPrivileges { user_id: 7 })
                .is_empty()
        );
    }
}
