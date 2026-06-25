use crate::dao::{DaoError, MessengerDao, PlayerDao};
use crate::game::messenger::{Messenger, MessengerEffect, MessengerFriend, MessengerLocation};
use crate::game::player::PlayerManager;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MessengerFriendRefreshExecutor;

impl MessengerFriendRefreshExecutor {
    pub fn refresh_friend_list(
        messenger_dao: &impl MessengerDao,
        player_dao: &impl PlayerDao,
        player_manager: &PlayerManager,
        user_id: i32,
        offline_user_id: Option<i32>,
    ) -> Result<MessengerEffect, DaoError> {
        let mut friends = Vec::new();

        for friend in messenger_dao.friends(user_id)? {
            let Some(details) = player_dao.details_by_id(friend.user_id())? else {
                continue;
            };
            let online = player_manager.get_by_id(details.id()).is_some();
            let location = online.then(|| Messenger::location_text(MessengerLocation::HotelView));

            friends.push(MessengerFriend::new(
                details.id(),
                details.username(),
                details.personal_greeting(),
                location,
                details.last_online(),
                online,
                online,
            ));
        }

        let mut messenger = Messenger::new(user_id);
        messenger.load(friends, Vec::new());
        Ok(messenger.send_friends(offline_user_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::in_memory::{InMemoryMessengerDao, InMemoryPlayerDao};
    use crate::dao::{CreatePlayer, MessengerDao, PlayerDao};
    use crate::game::player::{PlayerDetails, PlayerSession};
    use crate::messages::OutgoingMessage;

    fn create_player(username: &str) -> CreatePlayer {
        CreatePlayer::new(
            username,
            "password",
            format!("{username}@example.com"),
            "mission",
            "figure",
            100,
            "M",
            "1970-01-01",
        )
    }

    fn set_profile(
        dao: &InMemoryPlayerDao,
        username: &str,
        greeting: &str,
        last_online: i64,
    ) -> PlayerDetails {
        let stored = dao.details_by_username(username).unwrap().unwrap();
        let mut details = PlayerDetails::new();
        details.fill_full(
            stored.id(),
            stored.username(),
            stored.mission(),
            stored.figure(),
            stored.pool_figure(),
            stored.email(),
            stored.rank(),
            stored.credits(),
            stored.sex(),
            stored.country(),
            stored.badge(),
            stored.birthday(),
            last_online,
            greeting,
            stored.tickets(),
        );
        details.set_password(stored.password());
        dao.update_player(&details).unwrap();
        details
    }

    #[test]
    fn rebuilds_buddy_list_from_dao_and_online_sessions() {
        let messenger_dao = InMemoryMessengerDao::new();
        let player_dao = InMemoryPlayerDao::new();
        player_dao.create_player(&create_player("owner")).unwrap();
        player_dao.create_player(&create_player("offline")).unwrap();
        player_dao.create_player(&create_player("online")).unwrap();

        let owner = player_dao.details_by_username("owner").unwrap().unwrap();
        let offline = set_profile(&player_dao, "offline", "gone", 20);
        let online = set_profile(&player_dao, "online", "here", 40);
        messenger_dao.new_friend(owner.id(), online.id()).unwrap();
        messenger_dao.new_friend(owner.id(), offline.id()).unwrap();

        let mut player_manager = PlayerManager::new(Vec::new());
        player_manager.insert(PlayerSession::new(77, 30000, online.clone()));

        let MessengerEffect::SendFriends(packet) =
            MessengerFriendRefreshExecutor::refresh_friend_list(
                &messenger_dao,
                &player_dao,
                &player_manager,
                owner.id(),
                Some(online.id()),
            )
            .unwrap()
        else {
            panic!("expected buddy-list refresh");
        };

        let mut response = packet.compose();
        assert_eq!(
            response.get(),
            format!(
                "#BUDDYLIST\r{}\toffline\tgone\r\t20\r{}\tonline\there\r\t40##",
                offline.id(),
                online.id()
            )
        );
    }
}
