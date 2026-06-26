use crate::game::item::interactors::{Interaction, ItemInteractionEffect};
use crate::game::item::Item;
use crate::game::room::model::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PoolChangeBoothInteractor;

impl PoolChangeBoothInteractor {
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

    pub fn close_ui(item: &Item) -> Vec<ItemInteractionEffect> {
        let mut effects = Self::open(item);

        if let Some(walk) = item
            .custom_data()
            .and_then(|custom_data| Position::parse(custom_data).ok())
        {
            effects.push(ItemInteractionEffect::SetCanWalk { can_walk: true });
            effects.push(ItemInteractionEffect::WalkTo {
                x: walk.x(),
                y: walk.y(),
            });
        }

        effects
    }
}

impl Interaction for PoolChangeBoothInteractor {
    fn on_stopped_walking(
        &self,
        item: &Item,
        _player_position: Position,
    ) -> Vec<ItemInteractionEffect> {
        let mut effects = Self::close(item);
        effects.push(ItemInteractionEffect::OpenPoolChangeBooth);
        effects.push(ItemInteractionEffect::SetCanWalk { can_walk: false });
        effects
    }
}

#[cfg(test)]
#[path = "pool_change_booth_interactor_tests.rs"]
mod tests;
