use crate::game::player::Player;
use crate::game::room::RoomLeaveEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomLeaveInventoryExecutor;

impl RoomLeaveInventoryExecutor {
    pub fn apply(player: &mut Player, effect: &RoomLeaveEffect) -> bool {
        match effect {
            RoomLeaveEffect::DisposeInventory { user_id } if *user_id == player.details().id() => {
                player.inventory_mut().dispose();
                true
            }
            _ => false,
        }
    }

    pub fn apply_all(player: &mut Player, effects: &[RoomLeaveEffect]) -> usize {
        effects
            .iter()
            .filter(|effect| Self::apply(player, effect))
            .count()
    }
}
