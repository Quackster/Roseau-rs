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
mod tests {
    use super::*;

    fn room_user() -> RoomUser {
        RoomUser::new(7, "alice", "hd-100", "hello", None::<String>)
    }

    #[test]
    fn applies_matching_room_user_status_effects() {
        let mut user = room_user();

        RoomUserRoomEffectExecutor::apply_all(
            7,
            &mut user,
            &[
                RoomEffect::SetRoomUserStatus {
                    user_id: 7,
                    key: "flatctrl".to_owned(),
                    value: "useradmin".to_owned(),
                },
                RoomEffect::MarkRoomUserForUpdate { user_id: 7 },
            ],
        );

        assert!(user.contains_status("flatctrl"));
        assert_eq!(
            user.statuses().get("flatctrl").map(|status| status.value()),
            Some("useradmin")
        );
        assert!(user.needs_update());
    }

    #[test]
    fn removes_matching_room_user_status() {
        let mut user = room_user();
        user.set_status("mod", "admin", true, -1);

        RoomUserRoomEffectExecutor::apply(
            7,
            &mut user,
            &RoomEffect::RemoveRoomUserStatus {
                user_id: 7,
                key: "mod".to_owned(),
            },
        );

        assert!(!user.contains_status("mod"));
    }

    #[test]
    fn applies_let_user_in_room_assignment() {
        let mut user = room_user();

        RoomUserRoomEffectExecutor::apply(
            7,
            &mut user,
            &RoomEffect::LetUserIn {
                user_id: 7,
                room_id: 42,
            },
        );

        assert_eq!(user.room_id(), 42);
    }

    #[test]
    fn ignores_effects_for_other_users() {
        let mut user = room_user();

        RoomUserRoomEffectExecutor::apply_all(
            7,
            &mut user,
            &[
                RoomEffect::SetRoomUserStatus {
                    user_id: 9,
                    key: "flatctrl".to_owned(),
                    value: "useradmin".to_owned(),
                },
                RoomEffect::LetUserIn {
                    user_id: 9,
                    room_id: 42,
                },
                RoomEffect::MarkRoomUserForUpdate { user_id: 9 },
            ],
        );

        assert!(!user.contains_status("flatctrl"));
        assert_eq!(user.room_id(), 0);
        assert!(!user.needs_update());
    }
}
