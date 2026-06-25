use crate::game::player::PlayerManager;
use crate::game::GameTickRuntimeEffect;
use crate::messages::outgoing::WalletBalance;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoseauGameTickRuntimeActionPlan {
    SyncPlayerCredits { user_id: i32, credits: i32 },
    ResolveConfiguredHost { host: String },
    Network(PlayerNetworkEffect),
}

impl RoseauGameTickRuntimeActionPlan {
    pub fn plan(
        action: &GameTickRuntimeEffect,
        raw_config_ip: &str,
        player_manager: &PlayerManager,
    ) -> Vec<Self> {
        match action {
            GameTickRuntimeEffect::SendCreditBalance {
                user_id,
                new_balance,
            } => {
                let packet = WalletBalance::new(*new_balance).compose().get();
                let sync_plan = Self::SyncPlayerCredits {
                    user_id: *user_id,
                    credits: *new_balance,
                };
                let mut connection_ids = player_manager
                    .players()
                    .values()
                    .filter(|session| session.details().id() == *user_id)
                    .map(|session| session.connection_id())
                    .collect::<Vec<_>>();
                connection_ids.sort_unstable();
                let mut plans = vec![sync_plan];
                plans.extend(connection_ids.into_iter().map(|connection_id| {
                    Self::Network(PlayerNetworkEffect::WriteResponse {
                        connection_id,
                        packet: packet.clone(),
                    })
                }));
                plans
            }
            GameTickRuntimeEffect::ResolveServerIp => vec![Self::ResolveConfiguredHost {
                host: raw_config_ip.to_owned(),
            }],
            GameTickRuntimeEffect::KickAfkUser { user_id } => {
                let mut connection_ids = player_manager
                    .players()
                    .values()
                    .filter(|session| session.details().id() == *user_id)
                    .map(|session| session.connection_id())
                    .collect::<Vec<_>>();
                connection_ids.sort_unstable();
                connection_ids
                    .into_iter()
                    .map(|connection_id| {
                        Self::Network(PlayerNetworkEffect::CloseConnection { connection_id })
                    })
                    .collect()
            }
        }
    }

    pub fn collect(
        actions: &[GameTickRuntimeEffect],
        raw_config_ip: &str,
        player_manager: &PlayerManager,
    ) -> Vec<Self> {
        actions
            .iter()
            .flat_map(|action| Self::plan(action, raw_config_ip, player_manager))
            .collect()
    }
}

#[cfg(test)]
#[path = "roseau_game_tick_runtime_action_plan_tests.rs"]
mod tests;
