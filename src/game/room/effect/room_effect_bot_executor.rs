use crate::dao::{DaoError, RoomDao};
use crate::game::player::Bot;
use crate::game::room::RoomEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEffectBotExecutor;

impl RoomEffectBotExecutor {
    pub fn apply(
        bots: &mut Vec<Bot>,
        room_dao: &impl RoomDao,
        effect: &RoomEffect,
    ) -> Result<Vec<Bot>, DaoError> {
        match effect {
            RoomEffect::LoadBots { room_id } => {
                let loaded = room_dao.bots(*room_id)?;
                bots.clear();
                bots.extend(loaded.iter().cloned());
                Ok(loaded)
            }
            RoomEffect::StartPublicServer { .. }
            | RoomEffect::ScheduleWalkTicks
            | RoomEffect::ScheduleEventTicks
            | RoomEffect::LoadPassiveObjects { .. }
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
            | RoomEffect::RemoveLoadedRoom { .. }
            | RoomEffect::SaveRights { .. } => Ok(Vec::new()),
        }
    }

    pub fn apply_all(
        bots: &mut Vec<Bot>,
        room_dao: &impl RoomDao,
        effects: &[RoomEffect],
    ) -> Result<Vec<Bot>, DaoError> {
        let mut loaded = Vec::new();

        for effect in effects {
            loaded.extend(Self::apply(bots, room_dao, effect)?);
        }

        Ok(loaded)
    }
}

#[cfg(test)]
#[path = "room_effect_bot_executor_tests.rs"]
mod tests;
