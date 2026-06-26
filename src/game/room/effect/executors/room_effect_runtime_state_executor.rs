use crate::game::item::Item;
use crate::game::player::Bot;
use crate::game::room::RoomEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEffectRuntimeStateExecutor;

impl RoomEffectRuntimeStateExecutor {
    pub fn apply(items: &mut Vec<Item>, bots: &mut Vec<Bot>, effect: &RoomEffect) -> bool {
        match effect {
            RoomEffect::ClearRuntimeData => {
                items.clear();
                bots.clear();
                true
            }
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
            | RoomEffect::RemoveLoadedRoom { .. }
            | RoomEffect::SaveRights { .. } => false,
        }
    }

    pub fn apply_all(items: &mut Vec<Item>, bots: &mut Vec<Bot>, effects: &[RoomEffect]) -> usize {
        effects
            .iter()
            .filter(|effect| Self::apply(items, bots, effect))
            .count()
    }
}

#[cfg(test)]
#[path = "room_effect_runtime_state_executor_tests.rs"]
mod tests;
