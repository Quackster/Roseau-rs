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
