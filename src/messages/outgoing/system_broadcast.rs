use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemBroadcast {
    message: String,
}

impl SystemBroadcast {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl OutgoingMessage for SystemBroadcast {
    fn write(&self, response: &mut NettyResponse) {
        response.init("SYSTEMBROADCAST");
        response.append_new_argument(&self.message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_system_broadcast_packet() {
        let mut response = SystemBroadcast::new("maintenance").compose();

        assert_eq!(response.get(), "#SYSTEMBROADCAST\rmaintenance##");
    }
}
