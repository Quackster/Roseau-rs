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
#[path = "show_program_tests.rs"]
mod tests;
