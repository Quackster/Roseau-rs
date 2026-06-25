use crate::game::player::{
    PlayerEffect, PlayerPasswordActionEffectPlan, PlayerPasswordActionNetworkPlan,
    PlayerPasswordActionOutcome,
};
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerPasswordActionReport {
    outcomes: Vec<PlayerPasswordActionOutcome>,
    network_effects: Vec<PlayerNetworkEffect>,
    player_effects: Vec<PlayerEffect>,
}

impl PlayerPasswordActionReport {
    pub fn from_outcomes(
        outcomes: impl Into<Vec<PlayerPasswordActionOutcome>>,
        connection_id: i32,
    ) -> Self {
        let outcomes = outcomes.into();
        let network_effects = PlayerPasswordActionNetworkPlan::plan_all(&outcomes, connection_id);
        let player_effects = PlayerPasswordActionEffectPlan::plan_all(&outcomes);

        Self {
            outcomes,
            network_effects,
            player_effects,
        }
    }

    pub fn outcomes(&self) -> &[PlayerPasswordActionOutcome] {
        &self.outcomes
    }

    pub fn network_effects(&self) -> &[PlayerNetworkEffect] {
        &self.network_effects
    }

    pub fn player_effects(&self) -> &[PlayerEffect] {
        &self.player_effects
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::player::{
        PlayerDetails, PlayerLoginOutcome, PlayerProfileUpdateOutcome, PlayerRegistrationOutcome,
    };

    fn details() -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(7, "alice", "mission", "figure");
        details
    }

    #[test]
    fn derives_network_and_player_effects_from_password_action_outcomes() {
        let outcomes = vec![
            PlayerPasswordActionOutcome::Login(PlayerLoginOutcome::authenticated(
                &details(),
                "secret",
                false,
                30001,
                30001,
                Some(42),
            )),
            PlayerPasswordActionOutcome::Registration(PlayerRegistrationOutcome::Created),
            PlayerPasswordActionOutcome::ProfileUpdate(PlayerProfileUpdateOutcome::Ignored),
        ];

        let report = PlayerPasswordActionReport::from_outcomes(outcomes.clone(), 11);

        assert_eq!(report.outcomes(), outcomes.as_slice());
        assert_eq!(
            report.network_effects(),
            &[PlayerNetworkEffect::WriteResponse {
                connection_id: 11,
                packet: "#OK##".to_owned(),
            }]
        );
        assert_eq!(
            report.player_effects(),
            &[
                PlayerEffect::CloseConnection { connection_id: 42 },
                PlayerEffect::UpdateLastLogin { user_id: 7 },
            ]
        );
    }

    #[test]
    fn failed_login_report_contains_error_packet_without_player_effects() {
        let report = PlayerPasswordActionReport::from_outcomes(
            [PlayerPasswordActionOutcome::Login(
                PlayerLoginOutcome::failed(),
            )],
            11,
        );

        assert_eq!(
            report.network_effects(),
            &[PlayerNetworkEffect::WriteResponse {
                connection_id: 11,
                packet: "#ERROR Login incorrect##".to_owned(),
            }]
        );
        assert!(report.player_effects().is_empty());
    }
}
