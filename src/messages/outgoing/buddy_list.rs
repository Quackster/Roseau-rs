use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuddyList {
    friends: Vec<BuddyListFriend>,
    offline_id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuddyListFriend {
    id: i32,
    name: String,
    greeting: String,
    location: Option<String>,
    last_seen: String,
}

impl BuddyList {
    pub fn new(
        friends: impl IntoIterator<Item = BuddyListFriend>,
        offline_id: Option<i32>,
    ) -> Self {
        Self {
            friends: friends.into_iter().collect(),
            offline_id,
        }
    }
}

impl BuddyListFriend {
    pub fn new(
        id: i32,
        name: impl Into<String>,
        greeting: impl Into<String>,
        location: Option<impl Into<String>>,
        last_seen: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            greeting: greeting.into(),
            location: location.map(Into::into),
            last_seen: last_seen.into(),
        }
    }
}

impl OutgoingMessage for BuddyList {
    fn write(&self, response: &mut NettyResponse) {
        response.init("BUDDYLIST");

        for friend in &self.friends {
            let force_offline = self.offline_id == Some(friend.id);

            response.append_new_argument(friend.id);
            response.append_tab_argument(&friend.name);
            response.append_tab_argument(&friend.greeting);

            if let Some(location) = &friend.location {
                if !force_offline {
                    response.append_new_argument(location);
                } else {
                    response.append_new_argument("");
                }
            } else {
                response.append_new_argument("");
            }

            response.append_tab_argument(&friend.last_seen);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_buddy_list_packet() {
        let mut response = BuddyList::new(
            [
                BuddyListFriend::new(1, "alice", "hi", Some("Cafe"), "now"),
                BuddyListFriend::new(2, "bob", "away", None::<String>, "yesterday"),
            ],
            Some(2),
        )
        .compose();

        assert_eq!(
            response.get(),
            "#BUDDYLIST\r1\talice\thi\rCafe\tnow\r2\tbob\taway\r\tyesterday##"
        );
    }
}
