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
