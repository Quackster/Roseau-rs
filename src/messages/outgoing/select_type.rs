use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SelectType;

impl OutgoingMessage for SelectType {
    fn write(&self, response: &mut NettyResponse) {
        response.init("SELECTTYPE");
        response.append_new_argument("x");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_select_type_packet() {
        let mut response = SelectType.compose();

        assert_eq!(response.get(), "#SELECTTYPE\rx##");
    }
}
