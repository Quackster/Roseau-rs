use crate::game::room::RoomEntryOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomEntryNetworkPlan;

impl RoomEntryNetworkPlan {
    pub fn plan(outcome: &RoomEntryOutcome, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        if let Some(packet) = outcome.flat_let_in() {
            return vec![Self::write(connection_id, packet.compose().get())];
        }

        outcome
            .error()
            .map(|packet| vec![Self::write(connection_id, packet.compose().get())])
            .unwrap_or_default()
    }

    pub fn plan_all(outcomes: &[RoomEntryOutcome], connection_id: i32) -> Vec<PlayerNetworkEffect> {
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
    use crate::game::room::RoomEffect;

    #[test]
    fn maps_let_in_to_current_connection_packet() {
        let effects = RoomEntryNetworkPlan::plan(&RoomEntryOutcome::LetIn, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#FLAT_LETIN##".to_owned(),
            }]
        );
    }

    #[test]
    fn maps_rejected_entry_to_current_connection_error() {
        let effects = RoomEntryNetworkPlan::plan(&RoomEntryOutcome::IncorrectPassword, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#ERROR Incorrect flat password##".to_owned(),
            }]
        );
    }

    #[test]
    fn leaves_doorbell_routing_to_room_effect_network_planning() {
        let effects = RoomEntryNetworkPlan::plan(
            &RoomEntryOutcome::Doorbell(vec![RoomEffect::SendDoorbell {
                user_id: 7,
                username: "alice".to_owned(),
            }]),
            42,
        );

        assert!(effects.is_empty());
    }
}
