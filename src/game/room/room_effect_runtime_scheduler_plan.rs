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
mod tests {
    use super::*;
    use crate::game::GameRuntimeTask;

    #[test]
    fn maps_room_tick_effects_to_runtime_scheduler_effects() {
        let scheduler_plan = GameRuntimeSchedulerPlan::java_default();

        let effects = RoomEffectRuntimeSchedulerPlan::plan_all(
            &[
                RoomEffect::ScheduleWalkTicks,
                RoomEffect::ScheduleEventTicks,
            ],
            42,
            &scheduler_plan,
        );

        assert_eq!(
            effects,
            vec![
                GameRuntimeSchedulerEffect::ScheduleFixedRate {
                    task: GameRuntimeTask::RoomWalkTick { room_id: 42 },
                    initial_delay_ms: 0,
                    interval_ms: 500,
                },
                GameRuntimeSchedulerEffect::ScheduleFixedRate {
                    task: GameRuntimeTask::RoomEventTick { room_id: 42 },
                    initial_delay_ms: 0,
                    interval_ms: 500,
                },
            ]
        );
    }

    #[test]
    fn maps_room_disposal_effects_to_runtime_scheduler_cancellation() {
        let scheduler_plan = GameRuntimeSchedulerPlan::java_default();

        let effects = RoomEffectRuntimeSchedulerPlan::plan_all(
            &[
                RoomEffect::ClearRuntimeData,
                RoomEffect::RemoveLoadedRoom { room_id: 77 },
            ],
            42,
            &scheduler_plan,
        );

        assert_eq!(
            effects,
            vec![
                GameRuntimeSchedulerEffect::CancelRoomTasks { room_id: 42 },
                GameRuntimeSchedulerEffect::CancelRoomTasks { room_id: 42 },
            ]
        );
    }

    #[test]
    fn ignores_non_scheduler_room_effects() {
        let scheduler_plan = GameRuntimeSchedulerPlan::java_default();

        let effects = RoomEffectRuntimeSchedulerPlan::plan_all(
            &[
                RoomEffect::SendOwnerPrivileges { user_id: 7 },
                RoomEffect::RegisterEvent {
                    event_name: "user_status".to_owned(),
                },
                RoomEffect::SaveRights {
                    room_id: 42,
                    rights: vec![7],
                },
            ],
            42,
            &scheduler_plan,
        );

        assert!(effects.is_empty());
    }
}
