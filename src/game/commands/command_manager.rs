use std::collections::HashMap;

use crate::game::commands::types::{AboutCommand, HelpCommand, SitCommand};
use crate::game::commands::{Command, CommandContext, CommandEffect};

pub struct CommandManager {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn load(&mut self) {
        self.commands
            .insert("about".to_owned(), Box::new(AboutCommand));
        self.commands.insert("sit".to_owned(), Box::new(SitCommand));
        self.commands
            .insert("help".to_owned(), Box::new(HelpCommand));
    }

    pub fn insert(&mut self, name: impl Into<String>, command: Box<dyn Command>) {
        self.commands.insert(name.into(), command);
    }

    pub fn has_command(&self, message: &str) -> bool {
        self.command_name(message)
            .is_some_and(|command_name| self.commands.contains_key(command_name))
    }

    pub fn invoke_command(&self, context: &CommandContext, message: &str) -> Vec<CommandEffect> {
        let Some(command_name) = self.command_name(message) else {
            return Vec::new();
        };

        self.commands
            .get(command_name)
            .map(|command| command.handle(context, message))
            .unwrap_or_default()
    }

    pub fn command_names(&self) -> Vec<&str> {
        self.commands.keys().map(String::as_str).collect()
    }

    fn command_name<'a>(&self, message: &'a str) -> Option<&'a str> {
        if !message.starts_with(':') || message.len() <= 1 {
            return None;
        }

        message.split(':').nth(1)
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
