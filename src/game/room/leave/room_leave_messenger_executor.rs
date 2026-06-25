use crate::game::messenger::{Messenger, MessengerEffect};
use crate::game::room::RoomLeaveEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomLeaveMessengerExecutor;

impl RoomLeaveMessengerExecutor {
    pub fn apply(messenger: &Messenger, effect: &RoomLeaveEffect) -> Vec<MessengerEffect> {
        match effect {
            RoomLeaveEffect::RefreshMainMessengerStatus { user_id }
                if *user_id == messenger.user_id() =>
            {
                messenger.status_effects(None)
            }
            _ => Vec::new(),
        }
    }

    pub fn apply_all(messenger: &Messenger, effects: &[RoomLeaveEffect]) -> Vec<MessengerEffect> {
        effects
            .iter()
            .flat_map(|effect| Self::apply(messenger, effect))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::messenger::MessengerFriend;

    fn friend(user_id: i32, online: bool, initialised: bool) -> MessengerFriend {
        MessengerFriend::new(
            user_id,
            format!("friend{user_id}"),
            "hello",
            Some("On Hotel View"),
            10,
            online,
            initialised,
        )
    }

    #[test]
    fn emits_refresh_effects_for_online_initialised_friends() {
        let mut messenger = Messenger::new(7);
        messenger.load(
            vec![
                friend(8, true, true),
                friend(9, true, false),
                friend(10, false, true),
            ],
            Vec::new(),
        );

        let effects = RoomLeaveMessengerExecutor::apply(
            &messenger,
            &RoomLeaveEffect::RefreshMainMessengerStatus { user_id: 7 },
        );

        assert_eq!(
            effects,
            vec![MessengerEffect::RefreshFriendList {
                user_id: 8,
                offline_user_id: None,
            }]
        );
    }

    #[test]
    fn ignores_non_matching_user_and_other_leave_effects() {
        let mut messenger = Messenger::new(7);
        messenger.load(vec![friend(8, true, true)], Vec::new());

        assert!(RoomLeaveMessengerExecutor::apply_all(
            &messenger,
            &[
                RoomLeaveEffect::RefreshMainMessengerStatus { user_id: 9 },
                RoomLeaveEffect::DisposeInventory { user_id: 7 },
            ],
        )
        .is_empty());
    }
}
