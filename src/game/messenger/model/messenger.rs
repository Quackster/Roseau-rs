use crate::dao::{DaoError, MessengerDao};
use crate::game::messenger::{MessengerEffect, MessengerFriend, MessengerLocation};
use crate::messages::outgoing::{BuddyAddRequests, BuddyList};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Messenger {
    user_id: i32,
    initialised: bool,
    friends: Vec<MessengerFriend>,
    requests: Vec<MessengerFriend>,
}

impl Messenger {
    pub fn new(user_id: i32) -> Self {
        Self {
            user_id,
            initialised: false,
            friends: Vec::new(),
            requests: Vec::new(),
        }
    }

    pub fn load_from_dao(&mut self, dao: &(impl MessengerDao + ?Sized)) -> Result<(), DaoError> {
        self.friends = dao
            .friends(self.user_id)?
            .into_iter()
            .map(|user| MessengerFriend::offline(user.user_id()))
            .collect();
        self.requests = dao
            .requests(self.user_id)?
            .into_iter()
            .map(|user| MessengerFriend::offline(user.user_id()))
            .collect();
        Ok(())
    }

    pub fn load(&mut self, friends: Vec<MessengerFriend>, requests: Vec<MessengerFriend>) {
        self.friends = friends;
        self.requests = requests;
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn has_request(&self, user_id: i32) -> bool {
        self.get_request(user_id).is_some()
    }

    pub fn is_friend(&self, user_id: i32) -> bool {
        self.get_friend(user_id).is_some()
    }

    pub fn get_friend(&self, user_id: i32) -> Option<&MessengerFriend> {
        self.friends
            .iter()
            .find(|friend| friend.user_id() == user_id)
    }

    pub fn get_request(&self, user_id: i32) -> Option<&MessengerFriend> {
        self.requests
            .iter()
            .find(|request| request.user_id() == user_id)
    }

    pub fn remove_friend(&mut self, user_id: i32) -> Option<MessengerFriend> {
        self.friends
            .iter()
            .position(|friend| friend.user_id() == user_id)
            .map(|index| self.friends.remove(index))
    }

    pub fn send_requests(&self) -> Option<MessengerEffect> {
        (!self.requests.is_empty()).then(|| {
            MessengerEffect::SendRequests(BuddyAddRequests::new(
                self.requests.iter().map(MessengerFriend::username),
            ))
        })
    }

    pub fn send_friends(&mut self, offline_user_id: Option<i32>) -> MessengerEffect {
        self.sort_friends_for_buddy_list();
        MessengerEffect::SendFriends(BuddyList::new(
            self.friends
                .iter()
                .map(MessengerFriend::buddy_list_entry)
                .collect::<Vec<_>>(),
            offline_user_id,
        ))
    }

    pub fn status_effects(&self, offline_user_id: Option<i32>) -> Vec<MessengerEffect> {
        self.friends
            .iter()
            .filter(|friend| friend.is_online() && friend.is_initialised())
            .map(|friend| MessengerEffect::RefreshFriendList {
                user_id: friend.user_id(),
                offline_user_id,
            })
            .collect()
    }

    pub fn dispose(&mut self) -> Vec<MessengerEffect> {
        let effects = self.status_effects(Some(self.user_id));
        self.friends.clear();
        self.requests.clear();
        self.initialised = false;
        effects
    }

    pub fn friends(&self) -> &[MessengerFriend] {
        &self.friends
    }

    pub fn requests(&self) -> &[MessengerFriend] {
        &self.requests
    }

    pub fn has_initialised(&self) -> bool {
        self.initialised
    }

    pub fn set_initialised(&mut self, initialised: bool) {
        self.initialised = initialised;
    }

    pub fn location_text(location: MessengerLocation) -> String {
        location.display_text()
    }

    fn sort_friends_for_buddy_list(&mut self) {
        self.friends.sort_by(|left, right| {
            let result = left.last_online().cmp(&right.last_online());
            if result == std::cmp::Ordering::Equal && (left.is_online() || right.is_online()) {
                left.is_online().cmp(&right.is_online())
            } else {
                result
            }
        });
    }
}

#[cfg(test)]
#[path = "messenger_tests.rs"]
mod tests;
