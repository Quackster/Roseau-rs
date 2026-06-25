use super::command_manager::*;
use crate::game::commands::types::ReloadDefinitionsCommand;

#[test]
fn loads_java_active_commands() {
    let mut manager = CommandManager::new();
    manager.load();

    assert!(manager.has_command(":about"));
    assert!(manager.has_command(":sit"));
    assert!(manager.has_command(":help"));
    assert!(!manager.has_command(":reloaddef"));
    assert!(!manager.has_command("about"));
    assert!(!manager.has_command(":about now"));
}

#[test]
fn invokes_registered_command_and_allows_optional_reload_command() {
    let mut manager = CommandManager::new();
    manager.load();
    manager.insert("reloaddef", Box::new(ReloadDefinitionsCommand));

    assert_eq!(
        manager.invoke_command(&CommandContext::new(), ":reloaddef"),
        vec![CommandEffect::ReloadItemDefinitions]
    );
}
