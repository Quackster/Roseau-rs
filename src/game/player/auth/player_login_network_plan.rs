use crate::game::player::PlayerLoginOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerLoginNetworkPlan;

impl PlayerLoginNetworkPlan {
    pub fn plan(outcome: &PlayerLoginOutcome, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        outcome
            .login_error()
            .map(|packet| {
                let mut response = packet.compose();
                vec![PlayerNetworkEffect::WriteResponse {
                    connection_id,
                    packet: response.get(),
                }]
            })
            .unwrap_or_default()
    }

    pub fn plan_all(
        outcomes: &[PlayerLoginOutcome],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, connection_id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::player::PlayerDetails;

    fn details() -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_full(
            7,
            "alice",
            "mission",
            "figure",
            "pool",
            "alice@example.test",
            1,
            10,
            "F",
            "UK",
            "",
            "1990-01-01",
            1234,
            "hello",
            2,
        );
        details
    }

    #[test]
    fn maps_failed_login_to_current_connection_error() {
        let effects = PlayerLoginNetworkPlan::plan(&PlayerLoginOutcome::failed(), 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#ERROR Login incorrect##".to_owned(),
            }]
        );
    }

    #[test]
    fn authenticated_login_has_no_direct_error_packet() {
        let outcome =
            PlayerLoginOutcome::authenticated(&details(), "secret", false, 30001, 30001, None);

        assert!(PlayerLoginNetworkPlan::plan(&outcome, 42).is_empty());
    }
}
