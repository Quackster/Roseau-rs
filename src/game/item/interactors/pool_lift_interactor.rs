use crate::game::item::interactors::{Interaction, ItemInteractionEffect};
use crate::game::item::Item;
use crate::game::room::model::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PoolLiftInteractor;

impl PoolLiftInteractor {
    pub fn open(item: &Item) -> Vec<ItemInteractionEffect> {
        vec![
            ItemInteractionEffect::ShowProgram {
                item_id: item.id(),
                program: "open".to_owned(),
            },
            ItemInteractionEffect::UnlockTiles { item_id: item.id() },
        ]
    }

    pub fn close(item: &Item) -> Vec<ItemInteractionEffect> {
        vec![
            ItemInteractionEffect::ShowProgram {
                item_id: item.id(),
                program: "close".to_owned(),
            },
            ItemInteractionEffect::LockTiles { item_id: item.id() },
        ]
    }

    pub fn jump_performance(
        username: impl Into<String>,
        data: impl Into<String>,
    ) -> Vec<ItemInteractionEffect> {
        vec![ItemInteractionEffect::SendJumpData {
            username: username.into(),
            data: data.into(),
        }]
    }
}

impl Interaction for PoolLiftInteractor {
    fn on_stopped_walking(
        &self,
        item: &Item,
        _player_position: Position,
    ) -> Vec<ItemInteractionEffect> {
        let mut effects = Self::close(item);
        effects.extend([
            ItemInteractionEffect::SendJumpingPlaceOk,
            ItemInteractionEffect::SetCanWalk { can_walk: false },
            ItemInteractionEffect::DecrementTickets { amount: 1 },
            ItemInteractionEffect::SendTickets,
            ItemInteractionEffect::SavePlayer,
        ]);
        effects
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;

    fn lift() -> Item {
        Item::new(
            11,
            1,
            1,
            "1",
            1,
            0.0,
            0,
            ItemDefinition::new(1, "poolLift", "", 1, 1, 0.0, "SF", "", "", ""),
            "",
            None,
        )
        .unwrap()
    }

    #[test]
    fn closes_lift_and_records_ticket_side_effects() {
        assert_eq!(
            PoolLiftInteractor.on_stopped_walking(&lift(), Position::new(1, 1, 0.0)),
            vec![
                ItemInteractionEffect::ShowProgram {
                    item_id: 11,
                    program: "close".to_owned(),
                },
                ItemInteractionEffect::LockTiles { item_id: 11 },
                ItemInteractionEffect::SendJumpingPlaceOk,
                ItemInteractionEffect::SetCanWalk { can_walk: false },
                ItemInteractionEffect::DecrementTickets { amount: 1 },
                ItemInteractionEffect::SendTickets,
                ItemInteractionEffect::SavePlayer,
            ]
        );
    }

    #[test]
    fn maps_jump_performance_to_jump_data_effect() {
        assert_eq!(
            PoolLiftInteractor::jump_performance("alice", "jump=1"),
            vec![ItemInteractionEffect::SendJumpData {
                username: "alice".to_owned(),
                data: "jump=1".to_owned(),
            }]
        );
    }
}
