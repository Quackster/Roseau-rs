use crate::game::commands::{CommandContext, CommandEffect};

pub trait Command {
    fn handle(&self, context: &CommandContext, message: &str) -> Vec<CommandEffect>;
}
