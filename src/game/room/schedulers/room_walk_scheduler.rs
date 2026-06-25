use crate::game::item::Item;
use crate::game::room::model::{calculate_direction, Position};
use crate::game::room::schedulers::{RoomWalkEntity, SchedulerEffect};
use crate::game::room::{RoomMapping, RoomOccupant};
use crate::util::display_two_place_value;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RoomWalkScheduler;

impl RoomWalkScheduler {
    pub fn tick(
        entities: &[RoomWalkEntity],
        mapping: &RoomMapping,
        items: &[Item],
        occupants: &[RoomOccupant],
        pool_figure_available: bool,
    ) -> Vec<SchedulerEffect> {
        if entities.is_empty() {
            return Vec::new();
        }

        let mut effects = Vec::new();
        let mut update_entities = Vec::new();

        for entity in entities {
            effects.extend(Self::process_entity(
                entity,
                mapping,
                items,
                occupants,
                pool_figure_available,
            ));

            if entity.needs_update_value()
                || effects
                    .iter()
                    .any(|effect| effect.entity_id() == Some(entity.entity_id()))
            {
                update_entities.push(entity.entity_id());
            }
        }

        if !update_entities.is_empty() {
            effects.push(SchedulerEffect::SendStatus(update_entities));
        }

        effects
    }

    fn process_entity(
        entity: &RoomWalkEntity,
        mapping: &RoomMapping,
        items: &[Item],
        occupants: &[RoomOccupant],
        pool_figure_available: bool,
    ) -> Vec<SchedulerEffect> {
        if !entity.is_walking() {
            return Vec::new();
        }

        let mut effects = vec![SchedulerEffect::SetLookResetTime {
            entity_id: entity.entity_id(),
            ticks: -1,
        }];

        let mut current_position = entity.position();

        if let Some(next) = entity.next() {
            current_position =
                Position::with_rotation(next.x(), next.y(), next.z(), next.rotation());
            effects.push(SchedulerEffect::MoveTo {
                entity_id: entity.entity_id(),
                position: current_position,
            });
            effects.push(SchedulerEffect::UpdateHeight {
                entity_id: entity.entity_id(),
                height: next.z(),
            });
        }

        let Some(next_step) = entity.path_values().front().copied() else {
            effects.push(SchedulerEffect::StopWalking {
                entity_id: entity.entity_id(),
            });
            effects.push(SchedulerEffect::TriggerCurrentItem {
                entity_id: entity.entity_id(),
                item_id: entity.current_item_id_value(),
            });
            effects.push(SchedulerEffect::MarkNeedsUpdate {
                entity_id: entity.entity_id(),
            });
            return effects;
        };

        if !mapping.is_valid_tile(
            entity.entity_id(),
            next_step.x(),
            next_step.y(),
            items,
            occupants,
            pool_figure_available,
        ) {
            effects.push(SchedulerEffect::ClearPath {
                entity_id: entity.entity_id(),
            });
            if let Some(goal) = entity.goal() {
                effects.push(SchedulerEffect::WalkTo {
                    entity_id: entity.entity_id(),
                    x: goal.x(),
                    y: goal.y(),
                });
            }
            return effects;
        }

        let rotation = calculate_direction(
            current_position.x(),
            current_position.y(),
            next_step.x(),
            next_step.y(),
        ) as i32;
        let height = mapping.stack_height(next_step.x(), next_step.y());

        effects.extend([
            SchedulerEffect::PopPath {
                entity_id: entity.entity_id(),
            },
            SchedulerEffect::RemoveStatus {
                entity_id: entity.entity_id(),
                key: "lay".to_owned(),
            },
            SchedulerEffect::RemoveStatus {
                entity_id: entity.entity_id(),
                key: "sit".to_owned(),
            },
            SchedulerEffect::SetRotation {
                entity_id: entity.entity_id(),
                rotation,
            },
            SchedulerEffect::SetStatus {
                entity_id: entity.entity_id(),
                key: "mv".to_owned(),
                value: format!(
                    " {},{},{}",
                    next_step.x(),
                    next_step.y(),
                    display_two_place_value(height)
                ),
                infinite: true,
                duration: -1,
            },
            SchedulerEffect::SetNext {
                entity_id: entity.entity_id(),
                position: next_step,
            },
            SchedulerEffect::MarkNeedsUpdate {
                entity_id: entity.entity_id(),
            },
        ]);

        effects
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    use crate::game::item::ItemDefinition;
    use crate::game::room::model::RoomModel;

    fn mapping() -> RoomMapping {
        RoomMapping::new(RoomModel::new("model_a", "000", 0, 0, 0, 0, false, false).unwrap())
    }

    fn chair() -> Item {
        Item::new(
            1,
            1,
            1,
            "1",
            0,
            0.0,
            0,
            ItemDefinition::new(1, "chair", "", 1, 1, 1.0, "SFC", "", "", ""),
            "",
            None,
        )
        .unwrap()
    }

    #[test]
    fn emits_movement_effects_for_next_path_step() {
        let mut mapping = mapping();
        let item = chair();
        mapping.regenerate_collision_maps([item.clone()]);
        let entity = RoomWalkEntity::new(7, Position::new(0, 0, 0.0))
            .walking(true)
            .path(VecDeque::from([Position::new(1, 0, 0.0)]));

        let effects = RoomWalkScheduler::tick(&[entity], &mapping, &[item], &[], false);

        assert!(effects.contains(&SchedulerEffect::SetLookResetTime {
            entity_id: 7,
            ticks: -1,
        }));
        assert!(effects.contains(&SchedulerEffect::PopPath { entity_id: 7 }));
        assert!(effects.contains(&SchedulerEffect::RemoveStatus {
            entity_id: 7,
            key: "lay".to_owned(),
        }));
        assert!(effects.contains(&SchedulerEffect::SetRotation {
            entity_id: 7,
            rotation: 2,
        }));
        assert!(effects.contains(&SchedulerEffect::SetStatus {
            entity_id: 7,
            key: "mv".to_owned(),
            value: " 1,0,1".to_owned(),
            infinite: true,
            duration: -1,
        }));
        assert_eq!(effects.last(), Some(&SchedulerEffect::SendStatus(vec![7])));
    }

    #[test]
    fn moves_to_existing_next_before_processing_path() {
        let mapping = mapping();
        let entity = RoomWalkEntity::new(7, Position::new(0, 0, 0.0))
            .walking(true)
            .with_next(Some(Position::with_rotation(1, 0, 2.0, 2)))
            .path(VecDeque::from([Position::new(2, 0, 0.0)]));

        let effects = RoomWalkScheduler::tick(&[entity], &mapping, &[], &[], false);

        assert!(effects.contains(&SchedulerEffect::MoveTo {
            entity_id: 7,
            position: Position::with_rotation(1, 0, 2.0, 2),
        }));
        assert!(effects.contains(&SchedulerEffect::UpdateHeight {
            entity_id: 7,
            height: 2.0,
        }));
    }

    #[test]
    fn clears_invalid_path_and_requests_repath_to_goal() {
        let mapping = mapping();
        let blocker = RoomOccupant::new(8, Position::new(1, 0, 0.0), None);
        let entity = RoomWalkEntity::new(7, Position::new(0, 0, 0.0))
            .walking(true)
            .with_goal(Some(Position::new(2, 0, 0.0)))
            .path(VecDeque::from([Position::new(1, 0, 0.0)]));

        let effects = RoomWalkScheduler::tick(&[entity], &mapping, &[], &[blocker], false);

        assert!(effects.contains(&SchedulerEffect::ClearPath { entity_id: 7 }));
        assert!(effects.contains(&SchedulerEffect::WalkTo {
            entity_id: 7,
            x: 2,
            y: 0,
        }));
    }

    #[test]
    fn stops_walking_when_path_is_empty() {
        let mapping = mapping();
        let entity = RoomWalkEntity::new(7, Position::new(0, 0, 0.0))
            .walking(true)
            .current_item_id(Some(12));

        let effects = RoomWalkScheduler::tick(&[entity], &mapping, &[], &[], false);

        assert!(effects.contains(&SchedulerEffect::StopWalking { entity_id: 7 }));
        assert!(effects.contains(&SchedulerEffect::TriggerCurrentItem {
            entity_id: 7,
            item_id: Some(12),
        }));
    }
}
