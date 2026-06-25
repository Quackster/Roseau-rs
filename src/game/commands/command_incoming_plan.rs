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
