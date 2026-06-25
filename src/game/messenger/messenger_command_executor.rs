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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::in_memory::{InMemoryMessengerDao, InMemoryPlayerDao};
    use crate::dao::{CreatePlayer, MessengerDao, PlayerDao};
    use crate::game::messenger::MessengerFriend;

    fn create_player(username: &str) -> CreatePlayer {
        CreatePlayer::new(
            username,
            "secret",
            format!("{username}@example.test"),
            "hello",
            "hd=100",
            50,
            "Male",
            "08.08.1997",
        )
    }

    fn player_dao() -> InMemoryPlayerDao {
        let dao = InMemoryPlayerDao::new();
        dao.create_player(&create_player("alice")).unwrap();
        dao.create_player(&create_player("bob")).unwrap();
        dao.create_player(&create_player("carol")).unwrap();
        dao
    }

    fn friend(user_id: i32, username: &str) -> MessengerFriend {
        MessengerFriend::new(user_id, username, "hello", None::<String>, 0, false, false)
    }

    #[test]
    fn creates_buddy_request_when_target_exists_and_is_not_already_requested() {
        let players = player_dao();
        let messenger_dao = InMemoryMessengerDao::new();
        let messenger = Messenger::new(1);

        let outcome =
            MessengerCommandExecutor::request_buddy(&players, &messenger_dao, &messenger, "bob")
                .unwrap();

        assert_eq!(
            outcome,
            MessengerCommandOutcome::BuddyRequestCreated { to_id: 2 }
        );
        assert!(messenger_dao.request_exists(1, 2).unwrap());
    }

    #[test]
    fn ignores_unknown_self_and_existing_buddy_requests() {
        let players = player_dao();
        let messenger_dao = InMemoryMessengerDao::new();
        let mut messenger = Messenger::new(1);
        messenger.load(Vec::new(), vec![friend(2, "bob")]);

        assert_eq!(
            MessengerCommandExecutor::request_buddy(
                &players,
                &messenger_dao,
                &messenger,
                "missing",
            )
            .unwrap(),
            MessengerCommandOutcome::Ignored
        );
        assert_eq!(
            MessengerCommandExecutor::request_buddy(&players, &messenger_dao, &messenger, "alice",)
                .unwrap(),
            MessengerCommandOutcome::Ignored
        );
        assert_eq!(
            MessengerCommandExecutor::request_buddy(&players, &messenger_dao, &messenger, "bob",)
                .unwrap(),
            MessengerCommandOutcome::Ignored
        );
        assert!(!messenger_dao.request_exists(1, 2).unwrap());
    }

    #[test]
    fn accepts_and_declines_existing_requests() {
        let players = player_dao();
        let messenger_dao = InMemoryMessengerDao::new();
        messenger_dao.new_request(2, 1).unwrap();
        messenger_dao.new_request(3, 1).unwrap();
        let mut messenger = Messenger::new(1);
        messenger.load(Vec::new(), vec![friend(2, "bob"), friend(3, "carol")]);

        let accepted =
            MessengerCommandExecutor::accept_buddy(&players, &messenger_dao, &messenger, "bob")
                .unwrap();
        let declined =
            MessengerCommandExecutor::decline_buddy(&players, &messenger_dao, &messenger, "carol")
                .unwrap();

        assert_eq!(
            accepted,
            MessengerCommandOutcome::BuddyAccepted { user_id: 2 }
        );
        assert_eq!(
            declined,
            MessengerCommandOutcome::BuddyDeclined { user_id: 3 }
        );
        assert!(messenger_dao
            .friends(1)
            .unwrap()
            .iter()
            .any(|user| user.user_id() == 2));
        assert!(!messenger_dao.request_exists(2, 1).unwrap());
        assert!(!messenger_dao.request_exists(3, 1).unwrap());
    }

    #[test]
    fn removes_existing_friend_only() {
        let players = player_dao();
        let messenger_dao = InMemoryMessengerDao::new();
        messenger_dao.new_friend(1, 2).unwrap();
        let mut messenger = Messenger::new(1);
        messenger.load(vec![friend(2, "bob")], Vec::new());

        let outcome =
            MessengerCommandExecutor::remove_buddy(&players, &messenger_dao, &messenger, "bob")
                .unwrap();

        assert_eq!(
            outcome,
            MessengerCommandOutcome::BuddyRemoved { user_id: 2 }
        );
        assert!(messenger_dao.friends(1).unwrap().is_empty());
        assert_eq!(
            MessengerCommandExecutor::remove_buddy(&players, &messenger_dao, &messenger, "carol")
                .unwrap(),
            MessengerCommandOutcome::Ignored
        );
    }

    #[test]
    fn sends_messages_only_to_current_friends() {
        let messenger_dao = InMemoryMessengerDao::new().with_current_time(1234);
        let mut messenger = Messenger::new(1);
        messenger.load(vec![friend(2, "bob")], Vec::new());

        let outcome =
            MessengerCommandExecutor::send_message(&messenger_dao, &messenger, &[2, 3], "hello")
                .unwrap();

        assert_eq!(
            outcome,
            MessengerCommandOutcome::MessagesSent {
                message_ids: vec![1]
            }
        );
        let unread = messenger_dao.unread_messages(2).unwrap();
        assert_eq!(unread.len(), 1);
        assert_eq!(unread[0].message(), "hello");
        assert!(messenger_dao.unread_messages(3).unwrap().is_empty());
    }

    #[test]
    fn marks_read_only_for_current_users_unread_message() {
        let messenger_dao = InMemoryMessengerDao::new();
        let message_id = messenger_dao.new_message(1, 2, "hello").unwrap();
        let other_message_id = messenger_dao.new_message(1, 3, "other").unwrap();

        let ignored =
            MessengerCommandExecutor::mark_read(&messenger_dao, 2, other_message_id).unwrap();
        let outcome = MessengerCommandExecutor::mark_read(&messenger_dao, 2, message_id).unwrap();

        assert_eq!(ignored, MessengerCommandOutcome::Ignored);
        assert_eq!(
            outcome,
            MessengerCommandOutcome::MessageMarkedRead { message_id }
        );
        assert!(messenger_dao.unread_messages(2).unwrap().is_empty());
        assert_eq!(messenger_dao.unread_messages(3).unwrap().len(), 1);
    }
}
