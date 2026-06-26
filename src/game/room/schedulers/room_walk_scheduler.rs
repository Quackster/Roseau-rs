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
            let height = mapping.walking_height(next.x(), next.y(), items);
            current_position = Position::with_rotation(next.x(), next.y(), height, next.rotation());
            effects.push(SchedulerEffect::MoveTo {
                entity_id: entity.entity_id(),
                position: current_position,
            });
            effects.push(SchedulerEffect::UpdateHeight {
                entity_id: entity.entity_id(),
                height,
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
        let height = mapping.walking_height(next_step.x(), next_step.y(), items);
        let next_step = Position::with_rotation(next_step.x(), next_step.y(), height, rotation);

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
#[path = "room_walk_scheduler_tests.rs"]
mod tests;
