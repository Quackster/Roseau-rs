use crate::messages::outgoing::BuddyListFriend;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessengerFriend {
    user_id: i32,
    username: String,
    greeting: String,
    location: Option<String>,
    last_online: i64,
    online: bool,
    initialised: bool,
}

impl MessengerFriend {
    pub fn new(
        user_id: i32,
        username: impl Into<String>,
        greeting: impl Into<String>,
        location: Option<impl Into<String>>,
        last_online: i64,
        online: bool,
        initialised: bool,
    ) -> Self {
        Self {
            user_id,
            username: username.into(),
            greeting: greeting.into(),
            location: location.map(Into::into),
            last_online,
            online,
            initialised,
        }
    }

    pub fn offline(user_id: i32) -> Self {
        Self::new(user_id, "", "", None::<String>, 0, false, false)
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn greeting(&self) -> &str {
        &self.greeting
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }

    pub fn last_online(&self) -> i64 {
        self.last_online
    }

    pub fn is_online(&self) -> bool {
        self.online
    }

    pub fn is_initialised(&self) -> bool {
        self.initialised
    }

    pub fn buddy_list_entry(&self) -> BuddyListFriend {
        BuddyListFriend::new(
            self.user_id,
            &self.username,
            &self.greeting,
            self.online.then(|| self.location.clone()).flatten(),
            self.last_online.to_string(),
        )
    }
}
