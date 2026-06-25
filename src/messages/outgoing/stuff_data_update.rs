use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StuffDataUpdate {
    item_padding: String,
    item_id: i32,
    data_class: String,
    custom_data: String,
}

impl StuffDataUpdate {
    pub fn new(
        item_padding: impl Into<String>,
        item_id: i32,
        data_class: impl Into<String>,
        custom_data: impl Into<String>,
    ) -> Self {
        Self {
            item_padding: item_padding.into(),
            item_id,
            data_class: data_class.into(),
            custom_data: custom_data.into(),
        }
    }
}

impl OutgoingMessage for StuffDataUpdate {
    fn write(&self, response: &mut NettyResponse) {
        response.init("STUFFDATAUPDATE");
        response.append_new_argument(&self.item_padding);
        response.append(self.item_id);
        response.append_part_argument("");
        response.append_part_argument(&self.data_class);
        response.append_part_argument(&self.custom_data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_stuff_data_update_packet() {
        let mut response = StuffDataUpdate::new("i:", 7, "poster", "blue").compose();

        assert_eq!(response.get(), "#STUFFDATAUPDATE\ri:7//poster/blue##");
    }
}
