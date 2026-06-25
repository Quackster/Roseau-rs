use crate::game::commands::CommandEffect;
use crate::messages::outgoing::SystemBroadcast;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CommandEffectNetworkPlan;

impl CommandEffectNetworkPlan {
    pub fn plan(effect: &CommandEffect, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        match effect {
            CommandEffect::SendAlert(message) => {
                vec![PlayerNetworkEffect::WriteResponse {
                    connection_id,
                    packet: SystemBroadcast::new(message).compose().get(),
                }]
            }
            CommandEffect::ReloadItemDefinitions
            | CommandEffect::RemoveRoomStatus { .. }
            | CommandEffect::SetRoomStatus { .. }
            | CommandEffect::MarkRoomNeedsUpdate => Vec::new(),
        }
    }

    pub fn plan_all(effects: &[CommandEffect], connection_id: i32) -> Vec<PlayerNetworkEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::plan(effect, connection_id))
            .collect()
    }
}
