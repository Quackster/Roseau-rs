use crate::game::commands::CommandEffect;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CommandIncomingPlan;

impl CommandIncomingPlan {
    pub fn plan(effect: &IncomingExecutionEffect) -> Vec<CommandEffect> {
        let IncomingExecutionEffect::Command(command) = effect else {
            return Vec::new();
        };

        vec![command.clone()]
    }

    pub fn plan_all(effects: &[IncomingExecutionEffect]) -> Vec<CommandEffect> {
        effects.iter().flat_map(Self::plan).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_command_effects_from_incoming_effects() {
        let effects = CommandIncomingPlan::plan_all(&[
            IncomingExecutionEffect::GoAway,
            IncomingExecutionEffect::Command(CommandEffect::SendAlert("hello".to_owned())),
            IncomingExecutionEffect::Command(CommandEffect::ReloadItemDefinitions),
        ]);

        assert_eq!(
            effects,
            vec![
                CommandEffect::SendAlert("hello".to_owned()),
                CommandEffect::ReloadItemDefinitions,
            ]
        );
    }

    #[test]
    fn preserves_room_status_command_effects() {
        let effects = CommandIncomingPlan::plan(&IncomingExecutionEffect::Command(
            CommandEffect::SetRoomStatus {
                key: "sit".to_owned(),
                value: " 1.0".to_owned(),
                infinite: true,
                duration: -1,
            },
        ));

        assert_eq!(
            effects,
            vec![CommandEffect::SetRoomStatus {
                key: "sit".to_owned(),
                value: " 1.0".to_owned(),
                infinite: true,
                duration: -1,
            }]
        );
    }

    #[test]
    fn ignores_unrelated_incoming_effects() {
        assert!(CommandIncomingPlan::plan(&IncomingExecutionEffect::ResetAfkTimer).is_empty());
    }
}
