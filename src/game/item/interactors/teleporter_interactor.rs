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
mod tests {
    use super::*;
    use crate::game::item::ItemDefinition;

    fn teleporter(id: i32, room_id: i32, x: &str, y: i32) -> Item {
        Item::new(
            id,
            room_id,
            1,
            x,
            y,
            0.0,
            2,
            ItemDefinition::new(1, "teleport", "", 1, 1, 0.0, "SFX", "", "", "DOOROPEN"),
            "",
            Some("TRUE".to_owned()),
        )
        .unwrap()
    }

    #[test]
    fn schedules_same_room_transfer_and_exit() {
        let current = teleporter(1, 10, "1", 1);
        let target = teleporter(2, 10, "5", 6);

        let effects = TeleporterInteractor::teleport_between_items(&current, &target, true);

        assert_eq!(
            effects[0],
            ItemInteractionEffect::SetCanWalk { can_walk: false }
        );
        assert_eq!(
            effects[1],
            ItemInteractionEffect::SendDoorOut { item_id: 1 }
        );

        let ItemInteractionEffect::Schedule {
            delay_ms,
            effects: scheduled,
        } = &effects[2]
        else {
            panic!("expected scheduled teleporter effects");
        };

        assert_eq!(*delay_ms, GameVariables::DEFAULT_TELEPORTER_DELAY);
        assert!(scheduled.contains(&ItemInteractionEffect::SetPosition {
            position: target.position(),
        }));
        assert!(scheduled.contains(&ItemInteractionEffect::SendDoorIn { item_id: 2 }));
    }

    #[test]
    fn leave_teleporter_opens_door_and_walks_in_front() {
        let target = teleporter(2, 10, "5", 6);
        let effects = TeleporterInteractor::leave_teleporter(&target);

        assert_eq!(effects[1], ItemInteractionEffect::SendDoorIn { item_id: 2 });
        let ItemInteractionEffect::Schedule {
            delay_ms: _,
            effects: scheduled,
        } = &effects[2]
        else {
            panic!("expected scheduled leave effects");
        };
        assert!(scheduled.contains(&ItemInteractionEffect::WalkTo { x: 6, y: 6 }));
    }
}
