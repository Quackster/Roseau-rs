use crate::game::item::interactors::ItemInteractionEffect;
use crate::game::room::entity::{RoomUser, RoomUserEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemInteractionEffectExecutor;

impl ItemInteractionEffectExecutor {
    pub fn apply(user: &mut RoomUser, effect: &ItemInteractionEffect) -> Vec<RoomUserEffect> {
        match effect {
            ItemInteractionEffect::RemoveStatus { status } => {
                user.remove_status(status);
                Vec::new()
            }
            ItemInteractionEffect::SetStatus {
                status,
                value,
                persistent,
                ticks,
            } => {
                user.set_status(status, value, *persistent, i64::from(*ticks));
                Vec::new()
            }
            ItemInteractionEffect::SetBodyRotation { rotation } => {
                let mut position = user.position();
                position.set_rotation(*rotation);
                user.set_position(position);
                Vec::new()
            }
            ItemInteractionEffect::SetPosition { position } => {
                user.set_position(*position);
                Vec::new()
            }
            ItemInteractionEffect::SetCanWalk { can_walk } => {
                user.set_can_walk(*can_walk);
                Vec::new()
            }
            ItemInteractionEffect::SetWalking { walking } => {
                user.set_walking(*walking);
                Vec::new()
            }
            ItemInteractionEffect::ClearNextStep => {
                user.set_next(None);
                Vec::new()
            }
            ItemInteractionEffect::ForceStopWalking => {
                user.force_stop_walking();
                Vec::new()
            }
            ItemInteractionEffect::MarkNeedsUpdate => {
                user.set_needs_update(true);
                Vec::new()
            }
            ItemInteractionEffect::SetGoal { position } => {
                user.set_goal(Some(*position));
                Vec::new()
            }
            ItemInteractionEffect::TriggerCurrentItem => {
                vec![RoomUserEffect::TriggerCurrentItem {
                    item_id: user.current_item_id(),
                }]
            }
            ItemInteractionEffect::BuildPathToGoal
            | ItemInteractionEffect::WalkTo { .. }
            | ItemInteractionEffect::ShowProgram { .. }
            | ItemInteractionEffect::LockTiles { .. }
            | ItemInteractionEffect::UnlockTiles { .. }
            | ItemInteractionEffect::OpenPoolChangeBooth
            | ItemInteractionEffect::SendJumpingPlaceOk
            | ItemInteractionEffect::SendJumpData { .. }
            | ItemInteractionEffect::DecrementTickets { .. }
            | ItemInteractionEffect::SendTickets
            | ItemInteractionEffect::SavePlayer
            | ItemInteractionEffect::SendDoorOut { .. }
            | ItemInteractionEffect::SendDoorIn { .. }
            | ItemInteractionEffect::LoadRoom { .. }
            | ItemInteractionEffect::LeaveRoom { .. }
            | ItemInteractionEffect::SetItemCustomData { .. }
            | ItemInteractionEffect::UpdateItemStatus { .. }
            | ItemInteractionEffect::Schedule { .. } => Vec::new(),
        }
    }

    pub fn apply_all(
        user: &mut RoomUser,
        effects: &[ItemInteractionEffect],
    ) -> Vec<RoomUserEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(user, effect))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::room::model::Position;

    fn room_user() -> RoomUser {
        let mut user = RoomUser::new(7, "alice", "hd-100", "hello", None::<String>);
        user.set_room_id(42);
        user
    }

    #[test]
    fn applies_furniture_stop_effects_to_room_user() {
        let mut user = room_user();
        user.set_status("dance", "", true, -1);
        user.set_status("lay", " 1 null", true, -1);

        ItemInteractionEffectExecutor::apply_all(
            &mut user,
            &[
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
            ],
        );

        assert_eq!(user.position().rotation(), 2);
        assert_eq!(user.position().head_rotation(), 2);
        assert!(!user.contains_status("dance"));
        assert!(!user.contains_status("lay"));
        assert!(user.contains_status("sit"));
    }

    #[test]
    fn applies_pool_ladder_state_effects() {
        let mut user = room_user();
        user.set_walking(true);
        user.set_next(Some(Position::new(1, 1, 0.0)));
        user.set_status("mv", " 1,1,0", true, -1);

        ItemInteractionEffectExecutor::apply_all(
            &mut user,
            &[
                ItemInteractionEffect::SetWalking { walking: false },
                ItemInteractionEffect::ClearNextStep,
                ItemInteractionEffect::ForceStopWalking,
                ItemInteractionEffect::SetPosition {
                    position: Position::new(2, 3, 0.0),
                },
                ItemInteractionEffect::SetGoal {
                    position: Position::new(4, 5, 0.0),
                },
                ItemInteractionEffect::SetWalking { walking: true },
                ItemInteractionEffect::MarkNeedsUpdate,
            ],
        );

        assert!(user.is_walking());
        assert_eq!(user.next(), None);
        assert!(!user.contains_status("mv"));
        assert_eq!(user.position(), Position::new(2, 3, 0.0));
        assert_eq!(user.goal(), Some(Position::new(4, 5, 0.0)));
        assert!(user.needs_update());
    }

    #[test]
    fn applies_booth_walk_lock_state_and_current_item_trigger() {
        let mut user = room_user();
        user.set_can_walk(true);
        user.set_current_item_id(Some(99));

        let effects = ItemInteractionEffectExecutor::apply_all(
            &mut user,
            &[
                ItemInteractionEffect::SetCanWalk { can_walk: false },
                ItemInteractionEffect::TriggerCurrentItem,
            ],
        );

        assert!(!user.can_walk());
        assert_eq!(
            effects,
            vec![RoomUserEffect::TriggerCurrentItem { item_id: Some(99) }]
        );
    }

    #[test]
    fn ignores_effects_for_other_runtime_boundaries() {
        let mut user = room_user();

        let effects = ItemInteractionEffectExecutor::apply_all(
            &mut user,
            &[
                ItemInteractionEffect::WalkTo { x: 1, y: 2 },
                ItemInteractionEffect::ShowProgram {
                    item_id: 3,
                    program: "open".to_owned(),
                },
                ItemInteractionEffect::SavePlayer,
            ],
        );

        assert!(effects.is_empty());
        assert_eq!(user.position(), Position::new(0, 0, 0.0));
    }
}
