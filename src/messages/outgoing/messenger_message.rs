use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessengerMessage {
    message_id: i32,
    from_id: i32,
    sent_at: String,
    message: String,
    figure: String,
}

impl MessengerMessage {
    pub fn new(
        message_id: i32,
        from_id: i32,
        sent_at: impl Into<String>,
        message: impl Into<String>,
        figure: impl Into<String>,
    ) -> Self {
        Self {
            message_id,
            from_id,
            sent_at: sent_at.into(),
            message: message.into(),
            figure: figure.into(),
        }
    }
}

impl OutgoingMessage for MessengerMessage {
    fn write(&self, response: &mut NettyResponse) {
        response.init("MESSENGER_MSG");
        response.append_new_argument(self.message_id);
        response.append_new_argument(self.from_id);
        response.append_new_argument("[]");
        response.append_new_argument(&self.sent_at);
        response.append_new_argument(&self.message);
        response.append_new_argument(&self.figure);
        response.append_new_argument("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_messenger_message_packet() {
        let mut response =
            MessengerMessage::new(9, 14, "2026-06-24 12:00", "hello", "hr-100").compose();

        assert_eq!(
            response.get(),
            "#MESSENGER_MSG\r9\r14\r[]\r2026-06-24 12:00\rhello\rhr-100\r##"
        );
    }
}
