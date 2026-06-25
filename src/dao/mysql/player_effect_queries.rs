use crate::dao::mysql::{PlayerQueries, SqlExecutionPlan};
use crate::game::player::PlayerEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerEffectQueries;

impl PlayerEffectQueries {
    pub fn plan(effect: &PlayerEffect, now: i64) -> Option<SqlExecutionPlan> {
        match effect {
            PlayerEffect::UpdateLastLogin { user_id } => {
                Some(Self::update_last_login_plan(*user_id, now))
            }
            PlayerEffect::SendAlert(_)
            | PlayerEffect::CloseConnection { .. }
            | PlayerEffect::CloseUserConnections { .. }
            | PlayerEffect::DisposeOwnedRooms { .. }
            | PlayerEffect::DisposeInventory { .. }
            | PlayerEffect::LeaveCurrentRoom { .. }
            | PlayerEffect::Messenger(_) => None,
        }
    }

    pub fn update_last_login_plan(user_id: i32, now: i64) -> SqlExecutionPlan {
        PlayerQueries::update_last_login(user_id, now).execute_plan()
    }
}
