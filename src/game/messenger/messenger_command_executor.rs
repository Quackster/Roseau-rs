use crate::dao::{DaoError, MessengerDao, PlayerDao};
use crate::game::messenger::Messenger;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MessengerCommandExecutor;

impl MessengerCommandExecutor {
    pub fn request_buddy(
        player_dao: &dyn PlayerDao,
        messenger_dao: &dyn MessengerDao,
        messenger: &Messenger,
        username: &str,
    ) -> Result<MessengerCommandOutcome, DaoError> {
        let Some(to_id) = player_dao.id_by_username(username)? else {
            return Ok(MessengerCommandOutcome::Ignored);
        };

        if to_id < 1 || to_id == messenger.user_id() || messenger.has_request(to_id) {
            return Ok(MessengerCommandOutcome::Ignored);
        }

        if messenger_dao.new_request(messenger.user_id(), to_id)? {
            Ok(MessengerCommandOutcome::BuddyRequestCreated { to_id })
        } else {
            Ok(MessengerCommandOutcome::Ignored)
        }
    }

    pub fn accept_buddy(
        player_dao: &dyn PlayerDao,
        messenger_dao: &dyn MessengerDao,
        messenger: &Messenger,
        username: &str,
    ) -> Result<MessengerCommandOutcome, DaoError> {
        let Some(from_id) = player_dao.id_by_username(username)? else {
            return Ok(MessengerCommandOutcome::Ignored);
        };

        if from_id < 1 || from_id == messenger.user_id() || !messenger.has_request(from_id) {
            return Ok(MessengerCommandOutcome::Ignored);
        }

        messenger_dao.remove_request(from_id, messenger.user_id())?;
        messenger_dao.new_friend(messenger.user_id(), from_id)?;
        Ok(MessengerCommandOutcome::BuddyAccepted { user_id: from_id })
    }

    pub fn decline_buddy(
        player_dao: &dyn PlayerDao,
        messenger_dao: &dyn MessengerDao,
        messenger: &Messenger,
        username: &str,
    ) -> Result<MessengerCommandOutcome, DaoError> {
        let Some(from_id) = player_dao.id_by_username(username)? else {
            return Ok(MessengerCommandOutcome::Ignored);
        };

        if from_id < 1 || from_id == messenger.user_id() || !messenger.has_request(from_id) {
            return Ok(MessengerCommandOutcome::Ignored);
        }

        messenger_dao.remove_request(from_id, messenger.user_id())?;
        Ok(MessengerCommandOutcome::BuddyDeclined { user_id: from_id })
    }

    pub fn remove_buddy(
        player_dao: &dyn PlayerDao,
        messenger_dao: &dyn MessengerDao,
        messenger: &Messenger,
        username: &str,
    ) -> Result<MessengerCommandOutcome, DaoError> {
        let Some(friend_id) = player_dao.id_by_username(username)? else {
            return Ok(MessengerCommandOutcome::Ignored);
        };

        if friend_id < 1 || friend_id == messenger.user_id() || !messenger.is_friend(friend_id) {
            return Ok(MessengerCommandOutcome::Ignored);
        }

        messenger_dao.remove_friend(friend_id, messenger.user_id())?;
        Ok(MessengerCommandOutcome::BuddyRemoved { user_id: friend_id })
    }

    pub fn send_message(
        messenger_dao: &dyn MessengerDao,
        messenger: &Messenger,
        receiver_ids: &[i32],
        message: &str,
    ) -> Result<MessengerCommandOutcome, DaoError> {
        let mut message_ids = Vec::new();
        for receiver_id in receiver_ids {
            if messenger.is_friend(*receiver_id) {
                message_ids.push(messenger_dao.new_message(
                    messenger.user_id(),
                    *receiver_id,
                    message,
                )?);
            }
        }

        if message_ids.is_empty() {
            Ok(MessengerCommandOutcome::Ignored)
        } else {
            Ok(MessengerCommandOutcome::MessagesSent { message_ids })
        }
    }

    pub fn mark_read(
        messenger_dao: &dyn MessengerDao,
        user_id: i32,
        message_id: i32,
    ) -> Result<MessengerCommandOutcome, DaoError> {
        let can_mark = messenger_dao
            .unread_messages(user_id)?
            .iter()
            .any(|message| message.id() == message_id);

        if !can_mark {
            return Ok(MessengerCommandOutcome::Ignored);
        }

        messenger_dao.mark_message_read(message_id)?;
        Ok(MessengerCommandOutcome::MessageMarkedRead { message_id })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessengerCommandOutcome {
    BuddyRequestCreated { to_id: i32 },
    BuddyAccepted { user_id: i32 },
    BuddyDeclined { user_id: i32 },
    BuddyRemoved { user_id: i32 },
    MessagesSent { message_ids: Vec<i32> },
    MessageMarkedRead { message_id: i32 },
    Ignored,
}
