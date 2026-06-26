use crate::game::player::{Player, PlayerEffect};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerEffectInventoryExecutor;

impl PlayerEffectInventoryExecutor {
    pub fn apply(player: &mut Player, effect: &PlayerEffect) -> bool {
        match effect {
            PlayerEffect::DisposeInventory { user_id } if *user_id == player.details().id() => {
                player.inventory_mut().dispose();
                true
            }
            _ => false,
        }
    }

    pub fn apply_all(player: &mut Player, effects: &[PlayerEffect]) -> usize {
        effects
            .iter()
            .filter(|effect| Self::apply(player, effect))
            .count()
    }
}

#[cfg(test)]
#[path = "player_effect_inventory_executor_tests.rs"]
mod tests;
