#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessengerMessage {
    id: i32,
    to_id: i32,
    from_id: i32,
    time_sent: i64,
    message: String,
}

impl MessengerMessage {
    pub fn new(
        id: i32,
        to_id: i32,
        from_id: i32,
        time_sent: i64,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id,
            to_id,
            from_id,
            time_sent,
            message: message.into(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn to_id(&self) -> i32 {
        self.to_id
    }

    pub fn from_id(&self) -> i32 {
        self.from_id
    }

    pub fn time_sent(&self) -> i64 {
        self.time_sent
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_messenger_message_fields() {
        let message = MessengerMessage::new(1, 2, 3, 12345, "hello");

        assert_eq!(message.id(), 1);
        assert_eq!(message.to_id(), 2);
        assert_eq!(message.from_id(), 3);
        assert_eq!(message.time_sent(), 12345);
        assert_eq!(message.message(), "hello");
    }
}
