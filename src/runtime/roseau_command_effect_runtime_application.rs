use crate::dao::{DaoError, ItemDao};
use crate::game::commands::{CommandEffect, CommandEffectExecutor};
use crate::runtime::RoseauApplicationRuntime;

impl RoseauApplicationRuntime {
    pub fn apply_command_effects(
        &mut self,
        item_dao: &impl ItemDao,
        effects: &[CommandEffect],
    ) -> Result<(), DaoError> {
        CommandEffectExecutor::apply_all(self.game_mut().item_manager_mut(), item_dao, effects)
    }
}
