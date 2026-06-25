use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderInfo {
    name: String,
    credits: i32,
}

impl OrderInfo {
    pub fn new(name: impl Into<String>, credits: i32) -> Self {
        Self {
            name: name.into(),
            credits,
        }
    }
}

impl OutgoingMessage for OrderInfo {
    fn write(&self, response: &mut NettyResponse) {
        response.init("ORDERINFO");
        response.append_new_argument(&self.name);
        response.append_new_argument(self.credits);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_order_info_packet() {
        let mut response = OrderInfo::new("chair", 3).compose();

        assert_eq!(response.get(), "#ORDERINFO\rchair\r3##");
    }
}
