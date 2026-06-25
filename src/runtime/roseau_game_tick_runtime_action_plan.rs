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
mod tests {
    use super::*;
    use crate::game::player::{PlayerDetails, PlayerSession};

    fn details(id: i32, username: &str) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(id, username, "mission", "figure");
        details
    }

    #[test]
    fn plans_host_resolution_and_afk_connection_closes() {
        let mut manager = PlayerManager::new(vec![]);
        manager.insert(PlayerSession::new(10, 37120, details(7, "alice")));
        manager.insert(PlayerSession::new(11, 37121, details(7, "alice")));
        manager.insert(PlayerSession::new(12, 37120, details(8, "bob")));

        assert_eq!(
            RoseauGameTickRuntimeActionPlan::collect(
                &[
                    GameTickRuntimeEffect::ResolveServerIp,
                    GameTickRuntimeEffect::KickAfkUser { user_id: 7 },
                ],
                "roseau.local",
                &manager,
            ),
            vec![
                RoseauGameTickRuntimeActionPlan::ResolveConfiguredHost {
                    host: "roseau.local".to_owned(),
                },
                RoseauGameTickRuntimeActionPlan::Network(PlayerNetworkEffect::CloseConnection {
                    connection_id: 10,
                }),
                RoseauGameTickRuntimeActionPlan::Network(PlayerNetworkEffect::CloseConnection {
                    connection_id: 11,
                }),
            ]
        );
    }

    #[test]
    fn plans_credit_balance_packets_for_matching_sessions() {
        let mut manager = PlayerManager::new(vec![]);
        manager.insert(PlayerSession::new(10, 37120, details(7, "alice")));
        manager.insert(PlayerSession::new(11, 37121, details(7, "alice")));
        manager.insert(PlayerSession::new(12, 37120, details(8, "bob")));

        assert_eq!(
            RoseauGameTickRuntimeActionPlan::plan(
                &GameTickRuntimeEffect::SendCreditBalance {
                    user_id: 7,
                    new_balance: 125,
                },
                "roseau.local",
                &manager,
            ),
            vec![
                RoseauGameTickRuntimeActionPlan::SyncPlayerCredits {
                    user_id: 7,
                    credits: 125,
                },
                RoseauGameTickRuntimeActionPlan::Network(PlayerNetworkEffect::WriteResponse {
                    connection_id: 10,
                    packet: "#WALLETBALANCE\r125##".to_owned(),
                }),
                RoseauGameTickRuntimeActionPlan::Network(PlayerNetworkEffect::WriteResponse {
                    connection_id: 11,
                    packet: "#WALLETBALANCE\r125##".to_owned(),
                }),
            ]
        );
    }

    #[test]
    fn returns_no_close_actions_for_missing_user() {
        let manager = PlayerManager::new(vec![]);

        assert!(RoseauGameTickRuntimeActionPlan::plan(
            &GameTickRuntimeEffect::KickAfkUser { user_id: 7 },
            "roseau.local",
            &manager,
        )
        .is_empty());
    }
}
