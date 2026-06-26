use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberInfo {
    name: String,
    greeting: String,
    last_seen: String,
    location: String,
    figure: String,
}

impl MemberInfo {
    pub fn new(
        name: impl Into<String>,
        greeting: impl Into<String>,
        last_seen: impl Into<String>,
        location: impl Into<String>,
        figure: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            greeting: greeting.into(),
            last_seen: last_seen.into(),
            location: location.into(),
            figure: figure.into(),
        }
    }
}

impl OutgoingMessage for MemberInfo {
    fn write(&self, response: &mut NettyResponse) {
        response.init("MEMBERINFO");
        response.append_argument("");
        response.append_new_argument(&self.name);
        response.append_new_argument(&self.greeting);
        response.append_new_argument(&self.last_seen);
        response.append_new_argument(&self.location);
        response.append_new_argument(&self.figure);
    }
}

#[cfg(test)]
#[path = "member_info_tests.rs"]
mod tests;
