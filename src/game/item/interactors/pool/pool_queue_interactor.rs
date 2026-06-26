use crate::game::item::interactors::{Interaction, ItemInteractionEffect};
use crate::game::item::Item;
use crate::game::room::model::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PoolQueueInteractor;

impl Interaction for PoolQueueInteractor {
    fn on_stopped_walking(
        &self,
        item: &Item,
        _player_position: Position,
    ) -> Vec<ItemInteractionEffect> {
        item.custom_data()
            .and_then(|custom_data| Position::parse(custom_data).ok())
            .map(|position| {
                vec![ItemInteractionEffect::WalkTo {
                    x: position.x(),
                    y: position.y(),
                }]
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
#[path = "pool_queue_interactor_tests.rs"]
mod tests;
