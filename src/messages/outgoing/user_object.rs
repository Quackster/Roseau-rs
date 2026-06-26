use crate::messages::OutgoingMessage;
use crate::protocol::{NettyResponse, SerializableObject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserObject<T> {
    details: T,
}

impl<T> UserObject<T> {
    pub fn new(details: T) -> Self {
        Self { details }
    }
}

impl<T> OutgoingMessage for UserObject<T>
where
    T: SerializableObject,
{
    fn write(&self, response: &mut NettyResponse) {
        response.init("USEROBJECT");
        response.append_object(&self.details);
    }
}

#[cfg(test)]
#[path = "user_object_tests.rs"]
mod tests;
