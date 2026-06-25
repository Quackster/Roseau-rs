use crate::game::commands::{Command, CommandContext, CommandEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ReloadDefinitionsCommand;

impl Command for ReloadDefinitionsCommand {
    fn handle(&self, _context: &CommandContext, _message: &str) -> Vec<CommandEffect> {
        vec![CommandEffect::ReloadItemDefinitions]
    }
}
