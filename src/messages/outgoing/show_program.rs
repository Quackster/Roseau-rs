use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowProgram {
    parameters: Vec<String>,
}

impl ShowProgram {
    pub fn new(parameters: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            parameters: parameters.into_iter().map(Into::into).collect(),
        }
    }
}

impl OutgoingMessage for ShowProgram {
    fn write(&self, response: &mut NettyResponse) {
        response.init("SHOWPROGRAM");

        if let Some((first, rest)) = self.parameters.split_first() {
            response.append_new_argument(first);

            for parameter in rest {
                response.append_argument(parameter);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_show_program_packet() {
        let mut response = ShowProgram::new(["room", "a", "b"]).compose();

        assert_eq!(response.get(), "#SHOWPROGRAM\rroom a b##");
    }
}
