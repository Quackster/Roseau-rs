use crate::game::room::RoomEffect;
use crate::server::ServerListenPlan;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEffectServerListenPlan;

impl RoomEffectServerListenPlan {
    pub fn plan(effect: &RoomEffect, bind_ip: &str) -> Option<ServerListenPlan> {
        match effect {
            RoomEffect::StartPublicServer { port, .. } => {
                let port = u16::try_from(*port).ok()?;
                Some(ServerListenPlan::new(bind_ip, vec![port]))
            }
            RoomEffect::ScheduleWalkTicks
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
            | RoomEffect::RemoveLoadedRoom { .. }
            | RoomEffect::SaveRights { .. } => None,
        }
    }

    pub fn plan_all(effects: &[RoomEffect], bind_ip: &str) -> Vec<ServerListenPlan> {
        effects
            .iter()
            .filter_map(|effect| Self::plan(effect, bind_ip))
            .collect()
    }
}
