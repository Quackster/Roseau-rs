use crate::game::room::entity::RoomUser;
use crate::game::room::RoomEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomUserRoomEffectExecutor;

impl RoomUserRoomEffectExecutor {
    pub fn apply(user_id: i32, user: &mut RoomUser, effect: &RoomEffect) {
        match effect {
            RoomEffect::SetRoomUserStatus {
                user_id: effect_user_id,
                key,
                value,
            } if *effect_user_id == user_id => {
                user.set_status(key, value, true, -1);
            }
            RoomEffect::RemoveRoomUserStatus {
                user_id: effect_user_id,
                key,
            } if *effect_user_id == user_id => {
                user.remove_status(key);
            }
            RoomEffect::MarkRoomUserForUpdate {
                user_id: effect_user_id,
            } if *effect_user_id == user_id => {
                user.set_needs_update(true);
            }
            RoomEffect::LetUserIn {
                user_id: effect_user_id,
                room_id,
            } if *effect_user_id == user_id => {
                user.set_room_id(*room_id);
            }
            _ => {}
        }
    }

    pub fn apply_all(user_id: i32, user: &mut RoomUser, effects: &[RoomEffect]) {
        for effect in effects {
            Self::apply(user_id, user, effect);
        }
    }
}

#[cfg(test)]
#[path = "room_user_room_effect_executor_tests.rs"]
mod tests;
