use crate::game::moderation::ModerationEffect;
use crate::messages::outgoing::CryForHelp;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ModerationEffectNetworkPlan;

impl ModerationEffectNetworkPlan {
    pub fn plan(effect: &ModerationEffect) -> PlayerNetworkEffect {
        match effect {
            ModerationEffect::SendCallForHelp {
                moderator_connection_id,
                call,
            } => PlayerNetworkEffect::WriteResponse {
                connection_id: *moderator_connection_id,
                packet: CryForHelp::new(call.clone()).compose().get(),
            },
        }
    }

    pub fn plan_all(effects: &[ModerationEffect]) -> Vec<PlayerNetworkEffect> {
        effects.iter().map(Self::plan).collect()
    }
}
