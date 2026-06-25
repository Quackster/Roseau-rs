use super::*;
use crate::game::moderation::{CallForHelp, ModerationManager};
use crate::game::player::{Permission, PlayerDetails, PlayerSession};

fn details(id: i32, username: &str, rank: i32) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        id, username, "mission", "figure", "", "email", rank, 0, "M", "GB", "", "", 0, "", 0,
    );
    details
}

fn player_manager() -> PlayerManager {
    let mut player_manager = PlayerManager::new(vec![Permission::new(
        ModerationCommandExecutor::ANSWER_CALL_FOR_HELP_PERMISSION,
        true,
        5,
    )]);
    player_manager.insert(PlayerSession::new(10, 30000, details(1, "mod", 5)));
    player_manager.insert(PlayerSession::new(11, 30000, details(2, "guest", 1)));
    player_manager.insert(PlayerSession::new(12, 30001, details(3, "other", 7)));
    player_manager
}

#[test]
fn plans_private_room_call_for_help_from_raw_incoming_payload() {
    let effects = ModerationIncomingPlan::plan(
        &IncomingExecutionEffect::CryForHelp {
            message: "/Private Room: den;0;please;39\tden\tAlex".to_owned(),
        },
        &player_manager(),
        &ModerationManager::new(),
        30000,
        ModerationRoomContext::new(39, "den", RoomType::Private),
        "alice",
        "2026-06-25 12:00",
    );

    assert_eq!(
        effects,
        vec![ModerationEffect::SendCallForHelp {
            moderator_connection_id: 10,
            call: CallForHelp::new("den", "alice", "please", "2026-06-25 12:00"),
        }]
    );
}

#[test]
fn plans_public_room_call_for_help_from_raw_incoming_payload() {
    let effects = ModerationIncomingPlan::plan(
        &IncomingExecutionEffect::CryForHelp {
            message: "/Habbo Lido;0;splash;ignored".to_owned(),
        },
        &player_manager(),
        &ModerationManager::new(),
        30000,
        ModerationRoomContext::new(0, "Habbo Lido", RoomType::Public),
        "alice",
        "now",
    );

    assert_eq!(
        effects,
        vec![ModerationEffect::SendCallForHelp {
            moderator_connection_id: 10,
            call: CallForHelp::new("Habbo Lido", "alice", "splash", "now"),
        }]
    );
}

#[test]
fn ignores_unrelated_incoming_effects() {
    assert!(ModerationIncomingPlan::plan(
        &IncomingExecutionEffect::GoAway,
        &player_manager(),
        &ModerationManager::new(),
        30000,
        ModerationRoomContext::new(0, "Habbo Lido", RoomType::Public),
        "alice",
        "now",
    )
    .is_empty());
}
