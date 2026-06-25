#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessengerUser {
    user_id: i32,
    online: bool,
    in_room: bool,
}

impl MessengerUser {
    pub fn new(user_id: i32) -> Self {
        Self {
            user_id,
            online: false,
            in_room: false,
        }
    }

    pub fn with_presence(user_id: i32, online: bool, in_room: bool) -> Self {
        Self {
            user_id,
            online,
            in_room: online && in_room,
        }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn is_online(&self) -> bool {
        self.online
    }

    pub fn in_room(&self) -> bool {
        self.online && self.in_room
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offline_users_are_never_in_room() {
        let user = MessengerUser::with_presence(42, false, true);

        assert_eq!(user.user_id(), 42);
        assert!(!user.is_online());
        assert!(!user.in_room());
    }
}
