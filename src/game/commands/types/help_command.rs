use crate::game::commands::{Command, CommandContext, CommandEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HelpCommand;

impl Command for HelpCommand {
    fn handle(&self, _context: &CommandContext, _message: &str) -> Vec<CommandEffect> {
        vec![CommandEffect::SendAlert(
            "Commands:\n\n- :sit\n- :about\n- :help.".to_owned(),
        )]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_java_help_alert_text() {
        assert_eq!(
            HelpCommand.handle(&CommandContext::new(), ":help"),
            vec![CommandEffect::SendAlert(
                "Commands:\n\n- :sit\n- :about\n- :help.".to_owned()
            )]
        );
    }
}
