use crate::game::item::interactors::{Interaction, ItemInteractionEffect};
use crate::game::item::Item;
use crate::game::room::model::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BedInteractor;

impl BedInteractor {
    pub fn is_valid_pillow_tile(item: &Item, position: Position) -> bool {
        if !item.definition().behaviour().can_lay_on_top() {
            return false;
        }

        Self::valid_pillow_tiles(item)
            .into_iter()
            .any(|tile| tile.is_match(position))
    }

    pub fn valid_pillow_tiles(item: &Item) -> Vec<Position> {
        let item_position = item.position();
        let mut tiles = vec![Position::new(item_position.x(), item_position.y(), 0.0)];
        let mut pillow = Position::new(-1, -1, 0.0);

        if item_position.rotation() == 0 {
            pillow = Position::new(item_position.x() + 1, item_position.y(), 0.0);
        }

        if item_position.rotation() == 2 {
            pillow = Position::new(item_position.x(), item_position.y() + 1, 0.0);
        }

        tiles.push(pillow);
        tiles
    }
}

impl Interaction for BedInteractor {
    fn on_stopped_walking(
        &self,
        item: &Item,
        player_position: Position,
    ) -> Vec<ItemInteractionEffect> {
        if Self::is_valid_pillow_tile(item, player_position) {
            return vec![
                ItemInteractionEffect::SetBodyRotation {
                    rotation: item.position().rotation(),
                },
                ItemInteractionEffect::RemoveStatus {
                    status: "dance".to_owned(),
                },
                ItemInteractionEffect::RemoveStatus {
                    status: "lay".to_owned(),
                },
                ItemInteractionEffect::SetStatus {
                    status: "lay".to_owned(),
                    value: format!(" {} null", item.definition().height() + 1.5),
                    persistent: true,
                    ticks: -1,
                },
            ];
        }

        let target = Self::valid_pillow_tiles(item)
            .into_iter()
            .find(|tile| !tile.is_match(player_position))
            .unwrap_or(player_position);

        vec![
            ItemInteractionEffect::SetPosition { position: target },
            ItemInteractionEffect::TriggerCurrentItem,
        ]
    }
}

#[cfg(test)]
#[path = "bed_interactor_tests.rs"]
mod tests;
