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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;

    fn item() -> Item {
        Item::new(
            1,
            1,
            1,
            "1",
            1,
            0.0,
            0,
            ItemDefinition::new(1, "mat", "", 1, 1, 0.0, "SF", "", "", ""),
            "",
            None,
        )
        .unwrap()
    }

    #[test]
    fn clears_sitting_and_laying_statuses_on_stop() {
        assert_eq!(
            BlankInteractor.on_stopped_walking(&item(), Position::new(1, 1, 0.0)),
            vec![
                ItemInteractionEffect::RemoveStatus {
                    status: "sit".to_owned(),
                },
                ItemInteractionEffect::RemoveStatus {
                    status: "lay".to_owned(),
                },
            ]
        );
    }
}
