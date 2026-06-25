use std::cell::{Cell, RefCell};
use std::collections::HashSet;

use crate::dao::{DaoError, MessengerDao};
use crate::game::messenger::{MessengerMessage, MessengerUser};

#[derive(Debug, Clone, PartialEq, Eq)]
struct StoredMessage {
    message: MessengerMessage,
    unread: bool,
}

#[derive(Debug, Default)]
pub struct InMemoryMessengerDao {
    friendships: RefCell<HashSet<(i32, i32)>>,
    requests: RefCell<HashSet<(i32, i32)>>,
    messages: RefCell<Vec<StoredMessage>>,
    next_message_id: Cell<i32>,
    current_time: Cell<i64>,
}

impl InMemoryMessengerDao {
    pub fn new() -> Self {
        Self {
            friendships: RefCell::new(HashSet::new()),
            requests: RefCell::new(HashSet::new()),
            messages: RefCell::new(Vec::new()),
            next_message_id: Cell::new(1),
            current_time: Cell::new(0),
        }
    }

    pub fn with_current_time(mut self, current_time: i64) -> Self {
        self.current_time = Cell::new(current_time);
        self
    }

    pub fn set_current_time(&self, current_time: i64) {
        self.current_time.set(current_time);
    }

    pub fn is_empty(&self) -> bool {
        self.friendships.borrow().is_empty()
            && self.requests.borrow().is_empty()
            && self.messages.borrow().is_empty()
    }

    fn next_message_id(&self) -> i32 {
        let id = self.next_message_id.get();
        self.next_message_id.set(id + 1);
        id
    }

    fn friendship_key(left: i32, right: i32) -> (i32, i32) {
        if left <= right {
            (left, right)
        } else {
            (right, left)
        }
    }
}

impl MessengerDao for InMemoryMessengerDao {
    fn friends(&self, user_id: i32) -> Result<Vec<MessengerUser>, DaoError> {
        Ok(self
            .friendships
            .borrow()
            .iter()
            .filter_map(|(sender, receiver)| {
                if *sender == user_id {
                    Some(MessengerUser::new(*receiver))
                } else if *receiver == user_id {
                    Some(MessengerUser::new(*sender))
                } else {
                    None
                }
            })
            .collect())
    }

    fn requests(&self, user_id: i32) -> Result<Vec<MessengerUser>, DaoError> {
        Ok(self
            .requests
            .borrow()
            .iter()
            .filter_map(|(from_id, to_id)| {
                (*to_id == user_id).then_some(MessengerUser::new(*from_id))
            })
            .collect())
    }

    fn new_request(&self, from_id: i32, to_id: i32) -> Result<bool, DaoError> {
        if self.request_exists(from_id, to_id)? {
            return Ok(false);
        }

        self.requests.borrow_mut().insert((from_id, to_id));
        Ok(true)
    }

    fn remove_request(&self, from_id: i32, to_id: i32) -> Result<(), DaoError> {
        self.requests.borrow_mut().remove(&(from_id, to_id));
        Ok(())
    }

    fn new_friend(&self, sender: i32, receiver: i32) -> Result<bool, DaoError> {
        Ok(self
            .friendships
            .borrow_mut()
            .insert(Self::friendship_key(sender, receiver)))
    }

    fn remove_friend(&self, friend_id: i32, user_id: i32) -> Result<(), DaoError> {
        self.friendships
            .borrow_mut()
            .remove(&Self::friendship_key(friend_id, user_id));
        Ok(())
    }

    fn request_exists(&self, from_id: i32, to_id: i32) -> Result<bool, DaoError> {
        let requests = self.requests.borrow();
        Ok(requests.contains(&(from_id, to_id)) || requests.contains(&(to_id, from_id)))
    }

    fn new_message(&self, from_id: i32, to_id: i32, message: &str) -> Result<i32, DaoError> {
        let id = self.next_message_id();
        self.messages.borrow_mut().push(StoredMessage {
            message: MessengerMessage::new(id, to_id, from_id, self.current_time.get(), message),
            unread: true,
        });
        Ok(id)
    }

    fn unread_messages(&self, user_id: i32) -> Result<Vec<MessengerMessage>, DaoError> {
        Ok(self
            .messages
            .borrow()
            .iter()
            .filter(|stored| stored.unread && stored.message.to_id() == user_id)
            .map(|stored| stored.message.clone())
            .collect())
    }

    fn mark_message_read(&self, message_id: i32) -> Result<(), DaoError> {
        if let Some(stored) = self
            .messages
            .borrow_mut()
            .iter_mut()
            .find(|stored| stored.message.id() == message_id)
        {
            stored.unread = false;
        }
        Ok(())
    }
}
