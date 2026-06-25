use crate::game::item::interactors::{Interaction, ItemInteractionEffect};
use crate::game::item::Item;
use crate::game::room::model::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlankInteractor;

impl Interaction for BlankInteractor {
    fn on_stopped_walking(
        &self,
        _item: &Item,
        _player_position: Position,
    ) -> Vec<ItemInteractionEffect> {
        vec![
            ItemInteractionEffect::RemoveStatus {
                status: "sit".to_owned(),
            },
            ItemInteractionEffect::RemoveStatus {
                status: "lay".to_owned(),
            },
        ]
    }
}
