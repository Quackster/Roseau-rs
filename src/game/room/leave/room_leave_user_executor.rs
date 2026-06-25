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
