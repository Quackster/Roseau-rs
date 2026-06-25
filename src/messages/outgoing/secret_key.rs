use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretKey {
    key: String,
}

impl SecretKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

impl OutgoingMessage for SecretKey {
    fn write(&self, response: &mut NettyResponse) {
        response.init("SECRET_KEY");
        response.append_new_argument(&self.key);
    }
}

#[cfg(test)]
#[path = "secret_key_tests.rs"]
mod tests;
