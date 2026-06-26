use std::collections::VecDeque;

use crate::game::room::entity::{RoomUser, RoomUserEffect};
use crate::game::room::model::Position;
use crate::game::room::schedulers::SchedulerEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SchedulerEffectExecutor;

impl SchedulerEffectExecutor {
    pub fn apply(user: &mut RoomUser, effect: &SchedulerEffect) -> Vec<RoomUserEffect> {
        match effect {
            SchedulerEffect::SetHeadRotation {
                entity_id,
                rotation,
            } if *entity_id == user.entity_id() => {
                let mut position = user.position();
                position.set_head_rotation(*rotation);
                user.set_position(position);
                Vec::new()
            }
            SchedulerEffect::RemoveStatus { entity_id, key } if *entity_id == user.entity_id() => {
                user.remove_status(key);
                Vec::new()
            }
            SchedulerEffect::TickStatus { entity_id, key } if *entity_id == user.entity_id() => {
                user.tick_status(key);
                Vec::new()
            }
            SchedulerEffect::SetStatus {
                entity_id,
                key,
                value,
                infinite,
                duration,
            } if *entity_id == user.entity_id() => {
                user.set_status(key, value, *infinite, *duration);
                Vec::new()
            }
            SchedulerEffect::MarkNeedsUpdate { entity_id } if *entity_id == user.entity_id() => {
                user.set_needs_update(true);
                Vec::new()
            }
            SchedulerEffect::SendStatus(entity_ids) if entity_ids.contains(&user.entity_id()) => {
                user.set_needs_update(false);
                Vec::new()
            }
            SchedulerEffect::SetLookResetTime { entity_id, ticks }
                if *entity_id == user.entity_id() =>
            {
                user.set_look_reset_time(*ticks);
                Vec::new()
            }
            SchedulerEffect::SetTimeUntilNextDrink { entity_id, ticks }
                if *entity_id == user.entity_id() =>
            {
                user.set_time_until_next_drink(*ticks);
                Vec::new()
            }
            SchedulerEffect::SetRotation {
                entity_id,
                rotation,
            } if *entity_id == user.entity_id() => {
                let mut position = user.position();
                position.set_rotation(*rotation);
                user.set_position(position);
                Vec::new()
            }
            SchedulerEffect::MoveTo {
                entity_id,
                position,
            } if *entity_id == user.entity_id() => {
                user.set_position(*position);
                Vec::new()
            }
            SchedulerEffect::UpdateHeight { entity_id, height }
                if *entity_id == user.entity_id() =>
            {
                user.update_new_height(*height);
                Vec::new()
            }
            SchedulerEffect::SetNext {
                entity_id,
                position,
            } if *entity_id == user.entity_id() => {
                user.set_next(Some(*position));
                Vec::new()
            }
            SchedulerEffect::PopPath { entity_id } if *entity_id == user.entity_id() => {
                let mut path = user.path().clone();
                path.pop_front();
                user.set_path(path);
                Vec::new()
            }
            SchedulerEffect::ClearPath { entity_id } if *entity_id == user.entity_id() => {
                user.set_path(VecDeque::<Position>::new());
                Vec::new()
            }
            SchedulerEffect::StopWalking { entity_id } if *entity_id == user.entity_id() => {
                user.stop_walking()
            }
            SchedulerEffect::ShowProgram(_)
            | SchedulerEffect::TargetCamera { .. }
            | SchedulerEffect::SetCamera(_)
            | SchedulerEffect::WalkTo { .. }
            | SchedulerEffect::TriggerCurrentItem { .. }
            | SchedulerEffect::SendStatus(_)
            | SchedulerEffect::SetHeadRotation { .. }
            | SchedulerEffect::RemoveStatus { .. }
            | SchedulerEffect::TickStatus { .. }
            | SchedulerEffect::SetStatus { .. }
            | SchedulerEffect::MarkNeedsUpdate { .. }
            | SchedulerEffect::SetLookResetTime { .. }
            | SchedulerEffect::SetTimeUntilNextDrink { .. }
            | SchedulerEffect::SetRotation { .. }
            | SchedulerEffect::MoveTo { .. }
            | SchedulerEffect::UpdateHeight { .. }
            | SchedulerEffect::SetNext { .. }
            | SchedulerEffect::PopPath { .. }
            | SchedulerEffect::ClearPath { .. }
            | SchedulerEffect::StopWalking { .. } => Vec::new(),
        }
    }

    pub fn apply_all(user: &mut RoomUser, effects: &[SchedulerEffect]) -> Vec<RoomUserEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(user, effect))
            .collect()
    }
}

#[cfg(test)]
#[path = "scheduler_effect_executor_tests.rs"]
mod tests;
