use crate::game::player::PlayerManager;
use crate::game::room::RoomEffect;
use crate::messages::outgoing::{
    DoorbellRinging, FlatLetIn, YouAreController, YouAreNotController, YouAreOwner,
};
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEffectNetworkPlan;

impl RoomEffectNetworkPlan {
    pub fn plan(effect: &RoomEffect, player_manager: &PlayerManager) -> Vec<PlayerNetworkEffect> {
        match effect {
            RoomEffect::SendDoorbell { user_id, username } => {
                Self::write_to_user(player_manager, *user_id, DoorbellRinging::new(username))
            }
            RoomEffect::SendOwnerPrivileges { user_id } => {
                Self::write_to_user(player_manager, *user_id, YouAreOwner)
            }
            RoomEffect::SendControllerPrivileges { user_id } => {
                Self::write_to_user(player_manager, *user_id, YouAreController)
            }
            RoomEffect::SendNoControllerPrivileges { user_id } => {
                Self::write_to_user(player_manager, *user_id, YouAreNotController)
            }
            RoomEffect::LetUserIn { user_id, .. } => {
                Self::write_to_user(player_manager, *user_id, FlatLetIn)
            }
            RoomEffect::KickUser { user_id } => player_manager
                .get_by_id(*user_id)
                .map(|session| {
                    vec![PlayerNetworkEffect::CloseConnection {
                        connection_id: session.connection_id(),
                    }]
                })
                .unwrap_or_default(),
            RoomEffect::StartPublicServer { .. }
            | RoomEffect::ScheduleWalkTicks
            | RoomEffect::ScheduleEventTicks
            | RoomEffect::LoadPassiveObjects { .. }
            | RoomEffect::LoadBots { .. }
            | RoomEffect::RegenerateCollisionMaps
            | RoomEffect::RegisterEvent { .. }
            | RoomEffect::SetRoomUserStatus { .. }
            | RoomEffect::RemoveRoomUserStatus { .. }
            | RoomEffect::MarkRoomUserForUpdate { .. }
            | RoomEffect::LeaveRoom { .. }
            | RoomEffect::ClearRuntimeData
            | RoomEffect::RemoveLoadedRoom { .. }
            | RoomEffect::SaveRights { .. } => Vec::new(),
        }
    }

    pub fn plan_all(
        effects: &[RoomEffect],
        player_manager: &PlayerManager,
    ) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, player_manager))
            .collect()
    }

    fn write_to_user(
        player_manager: &PlayerManager,
        user_id: i32,
        message: impl OutgoingMessage,
    ) -> Vec<PlayerNetworkEffect> {
        player_manager
            .get_by_id(user_id)
            .map(|session| {
                let mut response = message.compose();
                vec![PlayerNetworkEffect::WriteResponse {
                    connection_id: session.connection_id(),
                    packet: response.get(),
                }]
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
#[path = "room_effect_network_plan_tests.rs"]
mod tests;
