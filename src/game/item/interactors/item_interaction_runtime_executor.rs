use crate::game::item::interactors::ItemInteractionRuntimeEffect;
use crate::game::player::PlayerManager;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemInteractionRuntimeExecutor;

impl ItemInteractionRuntimeExecutor {
    pub fn apply(
        player_manager: &mut PlayerManager,
        effect: &ItemInteractionRuntimeEffect,
    ) -> Vec<ItemInteractionRuntimeEffect> {
        match effect {
            ItemInteractionRuntimeEffect::SyncPlayerTickets { user_id, tickets } => {
                player_manager.sync_player_tickets(*user_id, *tickets);
                Vec::new()
            }
            ItemInteractionRuntimeEffect::ScheduleEffects { .. }
            | ItemInteractionRuntimeEffect::LoadRoom { .. }
            | ItemInteractionRuntimeEffect::LeaveRoom { .. } => vec![effect.clone()],
        }
    }

    pub fn apply_all(
        player_manager: &mut PlayerManager,
        effects: &[ItemInteractionRuntimeEffect],
    ) -> Vec<ItemInteractionRuntimeEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(player_manager, effect))
            .collect()
    }
}

#[cfg(test)]
mod tests {
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
}
