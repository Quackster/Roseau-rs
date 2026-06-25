use crate::game::item::interactors::{Interaction, ItemInteractionEffect};
use crate::game::item::Item;
use crate::game::room::model::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolLadderInteractor {
    enter_ladder: bool,
}

impl PoolLadderInteractor {
    pub fn new(enter_ladder: bool) -> Self {
        Self { enter_ladder }
    }

    pub fn enter_ladder(&self) -> bool {
        self.enter_ladder
    }

    fn pool_interactor(item: &Item, program: &str) -> Vec<ItemInteractionEffect> {
        let Some((warp, goal)) = parse_warp_and_goal(item.custom_data()) else {
            return Vec::new();
        };

        vec![
            ItemInteractionEffect::SetWalking { walking: false },
            ItemInteractionEffect::ClearNextStep,
            ItemInteractionEffect::ForceStopWalking,
            ItemInteractionEffect::SetPosition { position: warp },
            ItemInteractionEffect::MarkNeedsUpdate,
            ItemInteractionEffect::ShowProgram {
                item_id: item.id(),
                program: program.to_owned(),
            },
            ItemInteractionEffect::SetGoal { position: goal },
            ItemInteractionEffect::BuildPathToGoal,
            ItemInteractionEffect::SetWalking { walking: true },
        ]
    }
}

impl Interaction for PoolLadderInteractor {
    fn on_trigger(&self, item: &Item) -> Vec<ItemInteractionEffect> {
        let mut effects = Vec::new();
        if self.enter_ladder {
            effects.push(ItemInteractionEffect::SetStatus {
                status: "swim".to_owned(),
                value: String::new(),
                persistent: true,
                ticks: -1,
            });
            effects.extend(Self::pool_interactor(item, "enter"));
        } else {
            effects.push(ItemInteractionEffect::RemoveStatus {
                status: "swim".to_owned(),
            });
            effects.extend(Self::pool_interactor(item, "exit"));
        }

        effects
    }

    fn on_stopped_walking(
        &self,
        _item: &Item,
        _player_position: Position,
    ) -> Vec<ItemInteractionEffect> {
        Vec::new()
    }
}

impl Default for PoolLadderInteractor {
    fn default() -> Self {
        Self::new(false)
    }
}

fn parse_warp_and_goal(custom_data: Option<&str>) -> Option<(Position, Position)> {
    let (warp, goal) = custom_data?.split_once(' ')?;
    Some((Position::parse(warp).ok()?, Position::parse(goal).ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;

    fn ladder() -> Item {
        Item::new(
            13,
            1,
            1,
            "1",
            1,
            0.0,
            0,
            ItemDefinition::new(1, "poolEnter", "", 1, 1, 0.0, "SF", "", "", ""),
            "",
            Some("2,3 4,5".to_owned()),
        )
        .unwrap()
    }

    #[test]
    fn enter_ladder_sets_swim_and_builds_warp_effects() {
        let effects = PoolLadderInteractor::new(true).on_trigger(&ladder());

        assert_eq!(
            effects[0],
            ItemInteractionEffect::SetStatus {
                status: "swim".to_owned(),
                value: String::new(),
                persistent: true,
                ticks: -1,
            }
        );
        assert!(effects.contains(&ItemInteractionEffect::SetPosition {
            position: Position::new(2, 3, 0.0),
        }));
        assert!(effects.contains(&ItemInteractionEffect::SetGoal {
            position: Position::new(4, 5, 0.0),
        }));
        assert!(effects.contains(&ItemInteractionEffect::ShowProgram {
            item_id: 13,
            program: "enter".to_owned(),
        }));
    }

    #[test]
    fn exit_ladder_removes_swim() {
        let effects = PoolLadderInteractor::new(false).on_trigger(&ladder());

        assert_eq!(
            effects[0],
            ItemInteractionEffect::RemoveStatus {
                status: "swim".to_owned(),
            }
        );
        assert!(effects.contains(&ItemInteractionEffect::ShowProgram {
            item_id: 13,
            program: "exit".to_owned(),
        }));
    }
}
