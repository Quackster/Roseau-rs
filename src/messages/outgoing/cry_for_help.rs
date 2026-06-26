use crate::messages::OutgoingMessage;
use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryForHelp<T> {
    call: T,
}

impl<T> CryForHelp<T> {
    pub fn new(call: T) -> Self {
        Self { call }
    }
}

impl<T> OutgoingMessage for CryForHelp<T>
where
    T: SerializableObject,
{
    fn write(&self, response: &mut NettyResponse) {
        response.init("CRYFORHELP");
        response.append_object(&self.call);
    }
}

#[cfg(test)]
#[path = "cry_for_help_tests.rs"]
mod tests;
