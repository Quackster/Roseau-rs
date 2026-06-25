use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JumpData {
    name: String,
    data: String,
}

impl JumpData {
    pub fn new(name: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: data.into(),
        }
    }
}

impl OutgoingMessage for JumpData {
    fn write(&self, response: &mut NettyResponse) {
        response.init("JUMPDATA");
        response.append_new_argument(&self.name);
        response.append_new_argument(&self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_jump_data_packet() {
        let mut response = JumpData::new("hopper", "x=1").compose();

        assert_eq!(response.get(), "#JUMPDATA\rhopper\rx=1##");
    }
}
