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
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;

    fn booth() -> Item {
        Item::new(
            10,
            1,
            1,
            "1",
            1,
            0.0,
            0,
            ItemDefinition::new(1, "poolBooth", "", 1, 1, 0.0, "SF", "", "", ""),
            "",
            None,
        )
        .unwrap()
    }

    #[test]
    fn closes_booth_opens_ui_and_stops_walking() {
        assert_eq!(
            PoolChangeBoothInteractor.on_stopped_walking(&booth(), Position::new(1, 1, 0.0)),
            vec![
                ItemInteractionEffect::ShowProgram {
                    item_id: 10,
                    program: "close".to_owned(),
                },
                ItemInteractionEffect::LockTiles { item_id: 10 },
                ItemInteractionEffect::OpenPoolChangeBooth,
                ItemInteractionEffect::SetCanWalk { can_walk: false },
            ]
        );
    }

    #[test]
    fn close_ui_opens_booth_and_walks_to_custom_data_position() {
        let item = Item::new(
            10,
            1,
            1,
            "1",
            1,
            0.0,
            0,
            ItemDefinition::new(1, "poolBooth", "", 1, 1, 0.0, "SF", "", "", ""),
            "",
            Some("7,8".to_owned()),
        )
        .unwrap();

        assert_eq!(
            PoolChangeBoothInteractor::close_ui(&item),
            vec![
                ItemInteractionEffect::ShowProgram {
                    item_id: 10,
                    program: "open".to_owned(),
                },
                ItemInteractionEffect::UnlockTiles { item_id: 10 },
                ItemInteractionEffect::SetCanWalk { can_walk: true },
                ItemInteractionEffect::WalkTo { x: 7, y: 8 },
            ]
        );
    }

    #[test]
    fn close_ui_only_opens_booth_when_custom_data_position_is_invalid() {
        let item = booth();

        assert_eq!(
            PoolChangeBoothInteractor::close_ui(&item),
            vec![
                ItemInteractionEffect::ShowProgram {
                    item_id: 10,
                    program: "open".to_owned(),
                },
                ItemInteractionEffect::UnlockTiles { item_id: 10 },
            ]
        );
    }
}
