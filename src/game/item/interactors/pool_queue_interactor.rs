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
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;

    #[test]
    fn walks_to_custom_data_position() {
        let item = Item::new(
            12,
            1,
            1,
            "1",
            1,
            0.0,
            0,
            ItemDefinition::new(1, "poolQueue", "", 1, 1, 0.0, "SF", "", "", ""),
            "",
            Some("7,8".to_owned()),
        )
        .unwrap();

        assert_eq!(
            PoolQueueInteractor.on_stopped_walking(&item, Position::new(1, 1, 0.0)),
            vec![ItemInteractionEffect::WalkTo { x: 7, y: 8 }]
        );
    }
}
