use crate::game::messenger::MessengerEffect;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerEffectNetworkPlan;

impl MessengerEffectNetworkPlan {
    pub fn plan(effect: &MessengerEffect, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        match effect {
            MessengerEffect::SendRequests(packet) => {
                vec![Self::write(connection_id, packet.compose().get())]
            }
            MessengerEffect::SendFriends(packet) => {
                vec![Self::write(connection_id, packet.compose().get())]
            }
            MessengerEffect::RefreshFriendList { .. } => Vec::new(),
        }
    }

    pub fn plan_all(effects: &[MessengerEffect], connection_id: i32) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, connection_id))
            .collect()
    }

    fn write(connection_id: i32, packet: String) -> PlayerNetworkEffect {
        PlayerNetworkEffect::WriteResponse {
            connection_id,
            packet,
        }
    }
}
