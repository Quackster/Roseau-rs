use crate::game::player::PlayerCommandOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerCommandNetworkPlan;

impl PlayerCommandNetworkPlan {
    pub fn plan(outcome: &PlayerCommandOutcome, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        if let Some(packet) = outcome.user_object() {
            return vec![Self::write(connection_id, packet.compose().get())];
        }

        outcome
            .ph_tickets()
            .map(|packet| vec![Self::write(connection_id, packet.compose().get())])
            .unwrap_or_default()
    }

    pub fn plan_all(
        outcomes: &[PlayerCommandOutcome],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
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
            9,
        );
        details
    }

    #[test]
    fn maps_retrieve_user_info_to_current_connection_packet() {
        let effects = PlayerCommandNetworkPlan::plan(
            &PlayerCommandOutcome::retrieve_user_info(&details()),
            42,
        );

        assert_eq!(effects.len(), 1);
        assert_eq!(
            effects[0],
            PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#USEROBJECT\rname=alice\rfigure=hd-100\remail=alice@example.test\rbirthday=1990-01-01\rphonenumber=+44\rcustomData=hello\rhas_read_agreement=1\rsex=F\rcountry=UK\rhas_special_rights=0\rbadge_type=##".to_owned(),
            }
        );
    }

    #[test]
    fn maps_ticket_count_to_current_connection_packet() {
        let effects =
            PlayerCommandNetworkPlan::plan(&PlayerCommandOutcome::send_tickets(&details()), 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#PH_TICKETS 9##".to_owned(),
            }]
        );
    }
}
