use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuddyAddRequests {
    names: Vec<String>,
}

impl BuddyAddRequests {
    pub fn new(names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            names: names.into_iter().map(Into::into).collect(),
        }
    }
}

impl OutgoingMessage for BuddyAddRequests {
    fn write(&self, response: &mut NettyResponse) {
        response.init("BUDDYADDREQUESTS");
        response.append_new_argument("");

        for name in &self.names {
            response.append_part_argument(name);
        }
    }
}

#[cfg(test)]
#[path = "buddy_add_requests_tests.rs"]
mod tests;
