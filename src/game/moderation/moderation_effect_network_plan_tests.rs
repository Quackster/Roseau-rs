use super::moderation_effect_network_plan::*;
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
