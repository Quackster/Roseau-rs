use super::moderation_manager::*;

#[test]
fn builds_call_for_help_effects_for_moderators() {
    let manager = ModerationManager::new();
    let effects = manager.call_for_help([10, 11], "Lobby", "alice", "bad;message", "now");

    assert_eq!(effects.len(), 2);
    assert_eq!(
        effects[0],
        ModerationEffect::SendCallForHelp {
            moderator_connection_id: 10,
            call: CallForHelp::new("Lobby", "alice", "bad,message", "now"),
        }
    );
}
