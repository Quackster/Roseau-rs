use crate::game::player::FindUserOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FindUserNetworkPlan;

impl FindUserNetworkPlan {
    pub fn plan(outcome: &FindUserOutcome, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        if let Some(packet) = outcome.member_info() {
            return vec![Self::write(connection_id, packet.compose().get())];
        }

        outcome
            .no_such_user()
            .map(|packet| vec![Self::write(connection_id, packet.compose().get())])
            .unwrap_or_default()
    }

    pub fn plan_all(outcomes: &[FindUserOutcome], connection_id: i32) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, connection_id))
            .collect()
    }

    fn write(connection_id: i32, packet: String) -> PlayerNetworkEffect {
        PlayerNetworkEffect::WriteResponse {
            connection_id,
            packet,
        }
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
            "hello",
            "hd-100",
            "",
            "alice@example.test",
            1,
            50,
            "F",
            "UK",
            "",
            "1990-01-01",
            1234,
            "welcome",
            3,
        );
        details
    }

    #[test]
    fn maps_found_user_to_member_info_packet() {
        let outcome = FindUserOutcome::found(&details(), "now", "On Hotel View");

        let effects = FindUserNetworkPlan::plan(&outcome, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#MEMBERINFO \ralice\rwelcome\rnow\rOn Hotel View\rhd-100##".to_owned(),
            }]
        );
    }

    #[test]
    fn maps_missing_user_to_no_such_user_packet() {
        let effects = FindUserNetworkPlan::plan(&FindUserOutcome::Missing, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#NOSUCHUSER##".to_owned(),
            }]
        );
    }
}
