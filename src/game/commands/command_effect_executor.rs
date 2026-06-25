use crate::dao::{DaoError, ItemDao};
use crate::game::commands::CommandEffect;
use crate::game::item::ItemManager;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CommandEffectExecutor;

impl CommandEffectExecutor {
    pub fn apply(
        item_manager: &mut ItemManager,
        item_dao: &impl ItemDao,
        effect: &CommandEffect,
    ) -> Result<(), DaoError> {
        if matches!(effect, CommandEffect::ReloadItemDefinitions) {
            item_manager.load_definitions(item_dao.definitions()?.into_values());
        }

        Ok(())
    }

    pub fn apply_all(
        item_manager: &mut ItemManager,
        item_dao: &impl ItemDao,
        effects: &[CommandEffect],
    ) -> Result<(), DaoError> {
        for effect in effects {
            Self::apply(item_manager, item_dao, effect)?;
        }

        Ok(())
    }
}
