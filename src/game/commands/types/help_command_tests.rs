use super::help_command::*;

#[test]
fn creates_java_help_alert_text() {
    assert_eq!(
        HelpCommand.handle(&CommandContext::new(), ":help"),
        vec![CommandEffect::SendAlert(
            "Commands:\n\n- :sit\n- :about\n- :help.".to_owned()
        )]
    );
}
