use crate::game::commands::{Command, CommandContext, CommandEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AboutCommand;

impl Command for AboutCommand {
    fn handle(&self, _context: &CommandContext, _message: &str) -> Vec<CommandEffect> {
        vec![CommandEffect::SendAlert(
            "Roseau V1 server written by Quackster\n\nWith the help of:\n\n- Ascii\n- lab-hotel\n- Some stuff taken from office.boy and Nillus,\nthe authors of Blunk v5.".to_owned(),
        )]
    }
}

#[cfg(test)]
#[path = "about_command_tests.rs"]
mod tests;
