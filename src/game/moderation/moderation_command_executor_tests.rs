use super::*;
use crate::game::moderation::CallForHelp;
use crate::game::player::{Permission, PlayerDetails, PlayerSession};

fn details(id: i32, username: &str, rank: i32) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        id, username, "mission", "figure", "", "email", rank, 0, "M", "GB", "", "", 0, "", 0,
    );
    details
}

#[test]
fn sends_calls_only_to_main_server_moderators() {
    let mut player_manager = PlayerManager::new(vec![Permission::new(
        ModerationCommandExecutor::ANSWER_CALL_FOR_HELP_PERMISSION,
        true,
        5,
    )]);
    player_manager.insert(PlayerSession::new(10, 30000, details(1, "Moderator", 5)));
    player_manager.insert(PlayerSession::new(11, 30000, details(2, "User", 1)));
    player_manager.insert(PlayerSession::new(12, 30001, details(3, "OtherPort", 7)));
    let moderation_manager = ModerationManager::new();

    let effects = ModerationCommandExecutor::call_for_help(
        &player_manager,
        &moderation_manager,
        30000,
        "Lobby",
        "alice",
        "bad;message",
        "2026-06-25 09:00",
    );

    assert_eq!(
        effects,
        vec![ModerationEffect::SendCallForHelp {
            moderator_connection_id: 10,
            call: CallForHelp::new("Lobby", "alice", "bad,message", "2026-06-25 09:00"),
        }]
    );
}

#[test]
fn exact_rank_permission_is_honoured_for_moderators() {
    let mut player_manager = PlayerManager::new(vec![Permission::new(
        ModerationCommandExecutor::ANSWER_CALL_FOR_HELP_PERMISSION,
        false,
        4,
    )]);
    player_manager.insert(PlayerSession::new(20, 30000, details(1, "Exact", 4)));
    player_manager.insert(PlayerSession::new(21, 30000, details(2, "Higher", 5)));
    let moderation_manager = ModerationManager::new();

    let effects = ModerationCommandExecutor::call_for_help(
        &player_manager,
        &moderation_manager,
        30000,
        "Hallway",
        "bob",
        "help",
        "now",
    );

    assert_eq!(effects.len(), 1);
    assert_eq!(
        effects[0],
        ModerationEffect::SendCallForHelp {
            moderator_connection_id: 20,
            call: CallForHelp::new("Hallway", "bob", "help", "now"),
        }
    );
}
