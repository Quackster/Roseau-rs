use crate::game::room::RoomEffect;
use crate::game::{GameRuntimeSchedulerEffect, GameRuntimeSchedulerPlan};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEffectRuntimeSchedulerPlan;

impl RoomEffectRuntimeSchedulerPlan {
    pub fn plan(
        effect: &RoomEffect,
        room_id: i32,
        scheduler_plan: &GameRuntimeSchedulerPlan,
    ) -> Vec<GameRuntimeSchedulerEffect> {
        match effect {
            RoomEffect::ScheduleWalkTicks => scheduler_plan
                .schedule_room_ticks_effects(room_id)
                .into_iter()
                .take(1)
                .collect(),
            RoomEffect::ScheduleEventTicks => scheduler_plan
                .schedule_room_ticks_effects(room_id)
                .into_iter()
                .skip(1)
                .take(1)
                .collect(),
            RoomEffect::ClearRuntimeData | RoomEffect::RemoveLoadedRoom { .. } => {
                vec![scheduler_plan.cancel_room_tasks_effect(room_id)]
            }
            RoomEffect::StartPublicServer { .. }
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
            | RoomEffect::SaveRights { .. } => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[RoomEffect],
        room_id: i32,
        scheduler_plan: &GameRuntimeSchedulerPlan,
    ) -> Vec<GameRuntimeSchedulerEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, room_id, scheduler_plan))
            .collect()
    }
}

#[cfg(test)]
#[path = "room_effect_runtime_scheduler_plan_tests.rs"]
mod tests;
