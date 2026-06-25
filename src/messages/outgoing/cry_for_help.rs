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
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct HelpCall;

    impl SerializableObject for HelpCall {
        fn serialise(&self, response: &mut NettyResponse) {
            response.append_argument("alice");
        }
    }

    #[test]
    fn composes_cry_for_help_packet() {
        let mut response = CryForHelp::new(HelpCall).compose();

        assert_eq!(response.get(), "#CRYFORHELP alice##");
    }
}
