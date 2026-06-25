use super::*;
use crate::game::player::{PlayerDetails, PlayerSession};
use crate::game::room::model::Position;

fn details(id: i32, username: &str) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_basic(id, username, "mission", "figure");
    details.set_tickets(5);
    details
}

#[test]
fn applies_ticket_sync_to_matching_active_sessions() {
    let mut manager = PlayerManager::new(vec![]);
    manager.insert(PlayerSession::new(10, 37120, details(7, "alice")));
    manager.insert(PlayerSession::new(11, 37121, details(7, "alice")));
    manager.insert(PlayerSession::new(12, 37120, details(8, "bob")));

    let unapplied = ItemInteractionRuntimeExecutor::apply(
        &mut manager,
        &ItemInteractionRuntimeEffect::SyncPlayerTickets {
            user_id: 7,
            tickets: 4,
        },
    );

    assert!(unapplied.is_empty());
    assert_eq!(manager.players().get(&10).unwrap().details().tickets(), 4);
    assert_eq!(manager.players().get(&11).unwrap().details().tickets(), 4);
    assert_eq!(manager.players().get(&12).unwrap().details().tickets(), 5);
}

#[test]
fn leaves_scheduled_and_room_transfer_effects_for_runtime_boundary() {
    let mut manager = PlayerManager::new(vec![]);
    let effects = [
        ItemInteractionRuntimeEffect::ScheduleEffects {
            user_id: 7,
            delay_ms: 800,
            effects: Vec::new(),
        },
        ItemInteractionRuntimeEffect::LoadRoom {
            user_id: 7,
            room_id: 9,
            position: Position::new(1, 2, 0.0),
            rotation: 2,
        },
    ];

    let unapplied = ItemInteractionRuntimeExecutor::apply_all(&mut manager, &effects);

    assert_eq!(unapplied, effects);
}
