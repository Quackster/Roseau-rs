use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OpenUimakoppi;

impl OutgoingMessage for OpenUimakoppi {
    fn write(&self, response: &mut NettyResponse) {
        response.init("OPEN_UIMAKOPPI");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_open_uimakoppi_packet() {
        let mut response = OpenUimakoppi.compose();

        assert_eq!(response.get(), "#OPEN_UIMAKOPPI##");
    }
}
