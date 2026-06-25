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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::{SqlExecutionKind, SqlParameter};
    use crate::messages::outgoing::SystemBroadcast;

    #[test]
    fn maps_last_login_effect_to_player_update_plan() {
        let plan =
            PlayerEffectQueries::plan(&PlayerEffect::UpdateLastLogin { user_id: 7 }, 1234).unwrap();

        assert_eq!(plan.kind(), SqlExecutionKind::Execute);
        assert_eq!(plan.sql(), "UPDATE users SET last_online = ? WHERE id = ?");
        assert_eq!(
            plan.parameters(),
            &[SqlParameter::Long(1234), SqlParameter::Integer(7)]
        );
    }

    #[test]
    fn ignores_non_persistent_player_effects() {
        assert_eq!(
            PlayerEffectQueries::plan(
                &PlayerEffect::SendAlert(SystemBroadcast::new("maintenance")),
                1234,
            ),
            None
        );
        assert_eq!(
            PlayerEffectQueries::plan(&PlayerEffect::CloseConnection { connection_id: 9 }, 1234),
            None
        );
    }
}
