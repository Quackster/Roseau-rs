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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::moderation::CallForHelp;

    #[test]
    fn plans_call_for_help_packet_for_moderator_connection() {
        let effects = ModerationEffectNetworkPlan::plan_all(&[ModerationEffect::SendCallForHelp {
            moderator_connection_id: 42,
            call: CallForHelp::new("Lobby", "alice", "help", "2026-06-25 10:00"),
        }]);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet:
                    "#CRYFORHELP\rPrivate Room: Lobby @ 2026-06-25 10:00\rurl\rFrom: alice;0;Message: help##"
                        .to_owned(),
            }]
        );
    }
}
