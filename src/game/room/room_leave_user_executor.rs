use crate::game::room::entity::RoomUser;
use crate::game::room::RoomLeaveEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomLeaveUserExecutor;

impl RoomLeaveUserExecutor {
    pub fn apply(user: &mut RoomUser, effect: &RoomLeaveEffect) -> bool {
        match effect {
            RoomLeaveEffect::DisposeRoomUser { user_id } if *user_id == user.entity_id() => {
                user.dispose();
                true
            }
            _ => false,
        }
    }

    pub fn apply_all(user: &mut RoomUser, effects: &[RoomLeaveEffect]) -> usize {
        effects
            .iter()
            .filter(|effect| Self::apply(user, effect))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::room::model::Position;
    use crate::game::room::RoomLeaveEffect;

    fn room_user() -> RoomUser {
        let mut user = RoomUser::new(7, "alice", "hd=100", "hello", None::<String>);
        user.set_room_id(42);
        user.set_position(Position::with_rotation(3, 4, 1.0, 2));
        user.set_status("sit", " 1", true, -1);
        user.set_current_item_id(Some(99));
        user.set_needs_update(true);
        user
    }

    #[test]
    fn disposes_matching_room_user() {
        let mut user = room_user();

        let applied = RoomLeaveUserExecutor::apply(
            &mut user,
            &RoomLeaveEffect::DisposeRoomUser { user_id: 7 },
        );

        assert!(applied);
        assert_eq!(user.room_id(), 42);
        assert!(user.statuses().is_empty());
        assert_eq!(user.current_item_id(), None);
        assert!(!user.needs_update());
        assert_eq!(user.position(), Position::new(0, 0, 0.0));
    }

    #[test]
    fn ignores_non_matching_and_non_user_leave_effects() {
        let mut user = room_user();

        let count = RoomLeaveUserExecutor::apply_all(
            &mut user,
            &[
                RoomLeaveEffect::DisposeRoomUser { user_id: 8 },
                RoomLeaveEffect::BroadcastLogout {
                    username: "alice".to_owned(),
                },
            ],
        );

        assert_eq!(count, 0);
        assert!(user.contains_status("sit"));
        assert_eq!(user.current_item_id(), Some(99));
    }
}
