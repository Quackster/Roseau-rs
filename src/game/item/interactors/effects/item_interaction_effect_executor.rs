use crate::game::item::interactors::ItemInteractionEffect;
use crate::game::room::entity::{RoomUser, RoomUserEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemInteractionEffectExecutor;

impl ItemInteractionEffectExecutor {
    pub fn apply(user: &mut RoomUser, effect: &ItemInteractionEffect) -> Vec<RoomUserEffect> {
        match effect {
            ItemInteractionEffect::RemoveStatus { status } => {
                user.remove_status(status);
                Vec::new()
            }
            ItemInteractionEffect::SetStatus {
                status,
                value,
                persistent,
                ticks,
            } => {
                user.set_status(status, value, *persistent, i64::from(*ticks));
                Vec::new()
            }
            ItemInteractionEffect::SetBodyRotation { rotation } => {
                let mut position = user.position();
                position.set_rotation(*rotation);
                user.set_position(position);
                Vec::new()
            }
            ItemInteractionEffect::SetPosition { position } => {
                user.set_position(*position);
                Vec::new()
            }
            ItemInteractionEffect::SetCanWalk { can_walk } => {
                user.set_can_walk(*can_walk);
                Vec::new()
            }
            ItemInteractionEffect::SetWalking { walking } => {
                user.set_walking(*walking);
                Vec::new()
            }
            ItemInteractionEffect::ClearNextStep => {
                user.set_next(None);
                Vec::new()
            }
            ItemInteractionEffect::ForceStopWalking => {
                user.force_stop_walking();
                Vec::new()
            }
            ItemInteractionEffect::MarkNeedsUpdate => {
                user.set_needs_update(true);
                Vec::new()
            }
            ItemInteractionEffect::SetGoal { position } => {
                user.set_goal(Some(*position));
                Vec::new()
            }
            ItemInteractionEffect::TriggerCurrentItem => {
                vec![RoomUserEffect::TriggerCurrentItem {
                    item_id: user.current_item_id(),
                }]
            }
            ItemInteractionEffect::BuildPathToGoal
            | ItemInteractionEffect::WalkTo { .. }
            | ItemInteractionEffect::ShowProgram { .. }
            | ItemInteractionEffect::LockTiles { .. }
            | ItemInteractionEffect::UnlockTiles { .. }
            | ItemInteractionEffect::OpenPoolChangeBooth
            | ItemInteractionEffect::SendJumpingPlaceOk
            | ItemInteractionEffect::SendJumpData { .. }
            | ItemInteractionEffect::DecrementTickets { .. }
            | ItemInteractionEffect::SendTickets
            | ItemInteractionEffect::SavePlayer
            | ItemInteractionEffect::SendDoorOut { .. }
            | ItemInteractionEffect::SendDoorIn { .. }
            | ItemInteractionEffect::LoadRoom { .. }
            | ItemInteractionEffect::LeaveRoom { .. }
            | ItemInteractionEffect::SetItemCustomData { .. }
            | ItemInteractionEffect::UpdateItemStatus { .. }
            | ItemInteractionEffect::Schedule { .. } => Vec::new(),
        }
    }

    pub fn apply_all(
        user: &mut RoomUser,
        effects: &[ItemInteractionEffect],
    ) -> Vec<RoomUserEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(user, effect))
            .collect()
    }
}

#[cfg(test)]
#[path = "item_interaction_effect_executor_tests.rs"]
mod tests;
