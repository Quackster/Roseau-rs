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
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct PlayerDetails;

    impl SerializableObject for PlayerDetails {
        fn serialise(&self, response: &mut NettyResponse) {
            response.append_argument("alice");
        }
    }

    #[test]
    fn composes_user_object_packet() {
        let mut response = UserObject::new(PlayerDetails).compose();

        assert_eq!(response.get(), "#USEROBJECT alice##");
    }
}
