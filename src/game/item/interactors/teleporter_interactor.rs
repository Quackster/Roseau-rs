use crate::game::item::interactors::{Interaction, ItemInteractionEffect};
use crate::game::item::Item;
use crate::game::room::model::Position;
use crate::game::GameVariables;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TeleporterInteractor;

impl TeleporterInteractor {
    pub fn leave_teleporter(item: &Item) -> Vec<ItemInteractionEffect> {
        if !item.definition().behaviour().is_teleporter() {
            return Vec::new();
        }

        vec![
            ItemInteractionEffect::SetCanWalk { can_walk: false },
            ItemInteractionEffect::SendDoorIn { item_id: item.id() },
            ItemInteractionEffect::Schedule {
                delay_ms: GameVariables::DEFAULT_TELEPORTER_DELAY,
                effects: vec![
                    ItemInteractionEffect::SetItemCustomData {
                        item_id: item.id(),
                        custom_data: "TRUE".to_owned(),
                    },
                    ItemInteractionEffect::UpdateItemStatus { item_id: item.id() },
                    ItemInteractionEffect::SetCanWalk { can_walk: true },
                    ItemInteractionEffect::WalkTo {
                        x: item.position().square_in_front().x(),
                        y: item.position().square_in_front().y(),
                    },
                ],
            },
        ]
    }

    pub fn teleport_between_items(
        current: &Item,
        target: &Item,
        target_room_exists: bool,
    ) -> Vec<ItemInteractionEffect> {
        if !target_room_exists {
            return Vec::new();
        }

        let mut scheduled = Vec::new();
        if current.room_id() != target.room_id() {
            scheduled.push(ItemInteractionEffect::LeaveRoom {
                room_id: current.room_id(),
            });
            scheduled.push(ItemInteractionEffect::LoadRoom {
                room_id: target.room_id(),
                position: target.position(),
                rotation: target.position().rotation(),
            });
        } else {
            scheduled.push(ItemInteractionEffect::SetPosition {
                position: target.position(),
            });
            scheduled.extend(Self::leave_teleporter(target));
        }

        vec![
            ItemInteractionEffect::SetCanWalk { can_walk: false },
            ItemInteractionEffect::SendDoorOut {
                item_id: current.id(),
            },
            ItemInteractionEffect::Schedule {
                delay_ms: GameVariables::DEFAULT_TELEPORTER_DELAY,
                effects: scheduled,
            },
        ]
    }
}

impl Interaction for TeleporterInteractor {
    fn on_stopped_walking(
        &self,
        _item: &Item,
        _player_position: Position,
    ) -> Vec<ItemInteractionEffect> {
        Vec::new()
    }
}

#[cfg(test)]
#[path = "teleporter_interactor_tests.rs"]
mod tests;
