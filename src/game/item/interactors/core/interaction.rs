use crate::game::item::interactors::ItemInteractionEffect;
use crate::game::item::Item;
use crate::game::room::model::Position;

pub trait Interaction {
    fn on_trigger(&self, _item: &Item) -> Vec<ItemInteractionEffect> {
        Vec::new()
    }

    fn on_stopped_walking(
        &self,
        item: &Item,
        player_position: Position,
    ) -> Vec<ItemInteractionEffect>;
}
