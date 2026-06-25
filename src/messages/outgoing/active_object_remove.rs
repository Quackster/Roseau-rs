use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveObjectRemove {
    item_padding: String,
    item_id: i32,
}

impl ActiveObjectRemove {
    pub fn new(item_padding: impl Into<String>, item_id: i32) -> Self {
        Self {
            item_padding: item_padding.into(),
            item_id,
        }
    }
}

impl OutgoingMessage for ActiveObjectRemove {
    fn write(&self, response: &mut NettyResponse) {
        response.init("ACTIVEOBJECT_REMOVE");
        response.append_new_argument(&self.item_padding);
        response.append(self.item_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_active_object_remove_packet() {
        let mut response = ActiveObjectRemove::new("i:", 7).compose();

        assert_eq!(response.get(), "#ACTIVEOBJECT_REMOVE\ri:7##");
    }
}
