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
