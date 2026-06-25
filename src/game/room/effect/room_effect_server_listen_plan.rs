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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::ServerListenEffect;

    #[test]
    fn maps_public_room_start_effect_to_server_listen_plan() {
        let plan = RoomEffectServerListenPlan::plan(
            &RoomEffect::StartPublicServer {
                room_name: "lido".to_owned(),
                port: 37122,
            },
            "127.0.0.1",
        )
        .unwrap();

        assert_eq!(plan.bind_ip(), "127.0.0.1");
        assert_eq!(plan.ports(), &[37122]);
        assert_eq!(
            plan.listen_effects(),
            vec![
                ServerListenEffect::CreateCachedWorkerPools,
                ServerListenEffect::InstallPipelineStage { name: "encoder" },
                ServerListenEffect::InstallPipelineStage { name: "decoder" },
                ServerListenEffect::InstallPipelineStage { name: "handler" },
                ServerListenEffect::BindAddress {
                    address: "127.0.0.1:37122".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn ignores_invalid_ports_and_non_server_room_effects() {
        let plans = RoomEffectServerListenPlan::plan_all(
            &[
                RoomEffect::StartPublicServer {
                    room_name: "bad".to_owned(),
                    port: -1,
                },
                RoomEffect::SendOwnerPrivileges { user_id: 7 },
            ],
            "127.0.0.1",
        );

        assert!(plans.is_empty());
    }
}
