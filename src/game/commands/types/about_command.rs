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
mod tests {
    use super::*;

    #[test]
    fn creates_java_about_alert_text() {
        assert_eq!(
            AboutCommand.handle(&CommandContext::new(), ":about"),
            vec![CommandEffect::SendAlert(
                "Roseau V1 server written by Quackster\n\nWith the help of:\n\n- Ascii\n- lab-hotel\n- Some stuff taken from office.boy and Nillus,\nthe authors of Blunk v5.".to_owned(),
            )]
        );
    }
}
