use crate::game::player::PlayerRegistrationOutcome;
use crate::messages::outgoing::{BadName, Ok};
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerRegistrationNetworkPlan;

impl PlayerRegistrationNetworkPlan {
    pub fn plan(
        outcome: PlayerRegistrationOutcome,
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        match outcome {
            PlayerRegistrationOutcome::Created => {
                vec![Self::write(connection_id, Ok.compose().get())]
            }
            PlayerRegistrationOutcome::NameTaken => {
                vec![Self::write(connection_id, BadName.compose().get())]
            }
        }
    }

    pub fn plan_all(
        outcomes: &[PlayerRegistrationOutcome],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(*outcome, connection_id))
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

    #[test]
    fn maps_created_registration_to_ok_packet() {
        let effects = PlayerRegistrationNetworkPlan::plan(PlayerRegistrationOutcome::Created, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#OK##".to_owned(),
            }]
        );
    }

    #[test]
    fn maps_taken_name_to_bad_name_packet() {
        let effects = PlayerRegistrationNetworkPlan::plan(PlayerRegistrationOutcome::NameTaken, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#BADNAME##".to_owned(),
            }]
        );
    }
}
