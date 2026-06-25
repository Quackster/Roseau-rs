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
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;

    fn bed(rotation: i32) -> Item {
        Item::new(
            1,
            1,
            1,
            "5",
            6,
            0.0,
            rotation,
            ItemDefinition::new(1, "bed", "", 2, 1, 0.25, "SFB", "", "", ""),
            "",
            None,
        )
        .unwrap()
    }

    #[test]
    fn finds_java_pillow_tiles() {
        let item = bed(0);

        assert_eq!(
            BedInteractor::valid_pillow_tiles(&item),
            vec![Position::new(5, 6, 0.0), Position::new(6, 6, 0.0)]
        );
        assert!(BedInteractor::is_valid_pillow_tile(
            &item,
            Position::new(6, 6, 0.0)
        ));
    }

    #[test]
    fn lays_player_down_on_valid_pillow_tile() {
        let effects = BedInteractor.on_stopped_walking(&bed(2), Position::new(5, 7, 0.0));

        assert!(effects.contains(&ItemInteractionEffect::SetBodyRotation { rotation: 2 }));
        assert!(effects.contains(&ItemInteractionEffect::SetStatus {
            status: "lay".to_owned(),
            value: " 1.75 null".to_owned(),
            persistent: true,
            ticks: -1,
        }));
    }

    #[test]
    fn moves_player_to_pillow_tile_before_retriggering() {
        assert_eq!(
            BedInteractor.on_stopped_walking(&bed(0), Position::new(8, 9, 0.0)),
            vec![
                ItemInteractionEffect::SetPosition {
                    position: Position::new(5, 6, 0.0),
                },
                ItemInteractionEffect::TriggerCurrentItem,
            ]
        );
    }
}
