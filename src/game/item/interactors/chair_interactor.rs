use crate::game::item::interactors::{Interaction, ItemInteractionEffect};
use crate::game::item::Item;
use crate::game::room::model::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ChairInteractor;

impl ChairInteractor {
    pub fn has_valid_entry(
        item: &Item,
        entity_position: Position,
        neighbour: Position,
        front_is_valid: bool,
        left_is_valid: bool,
        right_is_valid: bool,
    ) -> bool {
        let front = item.position().square_in_front();
        let left = item.position().square_left();
        let right = item.position().square_right();
        let left_distance = left.distance(entity_position);
        let right_distance = right.distance(entity_position);

        if front_is_valid && !neighbour.is_match(front) {
            return false;
        }

        if left_distance <= right_distance {
            if left_is_valid && !neighbour.is_match(left) {
                return false;
            }
        } else if right_is_valid && !neighbour.is_match(right) {
            return false;
        }

        true
    }
}

impl Interaction for ChairInteractor {
    fn on_stopped_walking(
        &self,
        item: &Item,
        _player_position: Position,
    ) -> Vec<ItemInteractionEffect> {
        vec![
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
                status: "sit".to_owned(),
                value: format!(" {}", item.definition().height()),
                persistent: true,
                ticks: -1,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;

    fn chair() -> Item {
        Item::new(
            1,
            1,
            1,
            "5",
            6,
            0.0,
            2,
            ItemDefinition::new(1, "chair", "", 1, 1, 0.75, "SFC", "", "", ""),
            "",
            None,
        )
        .unwrap()
    }

    #[test]
    fn sets_sit_status_on_stop() {
        assert_eq!(
            ChairInteractor.on_stopped_walking(&chair(), Position::new(6, 6, 0.0)),
            vec![
                ItemInteractionEffect::SetBodyRotation { rotation: 2 },
                ItemInteractionEffect::RemoveStatus {
                    status: "dance".to_owned(),
                },
                ItemInteractionEffect::RemoveStatus {
                    status: "lay".to_owned(),
                },
                ItemInteractionEffect::SetStatus {
                    status: "sit".to_owned(),
                    value: " 0.75".to_owned(),
                    persistent: true,
                    ticks: -1,
                },
            ]
        );
    }

    #[test]
    fn validates_neighbour_entry_priority() {
        let item = chair();
        let entity_position = Position::new(5, 7, 0.0);

        assert!(ChairInteractor::has_valid_entry(
            &item,
            entity_position,
            item.position().square_in_front(),
            true,
            false,
            false,
        ));
        assert!(!ChairInteractor::has_valid_entry(
            &item,
            entity_position,
            item.position().square_right(),
            true,
            false,
            false,
        ));
    }
}
