use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoorbellRinging {
    name: String,
}

impl DoorbellRinging {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl OutgoingMessage for DoorbellRinging {
    fn write(&self, response: &mut NettyResponse) {
        response.init("DOORBELL_RINGING");
        response.append_new_argument(&self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_doorbell_ringing_packet() {
        let mut response = DoorbellRinging::new("alice").compose();

        assert_eq!(response.get(), "#DOORBELL_RINGING\ralice##");
    }
}
