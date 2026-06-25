use crate::game::player::PlayerNameApproval;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PlayerNameApprovalNetworkPlan;

impl PlayerNameApprovalNetworkPlan {
    pub fn plan(approval: PlayerNameApproval, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        if let Some(packet) = approval.name_approved() {
            return vec![Self::write(connection_id, packet.compose().get())];
        }

        approval
            .name_unacceptable()
            .map(|packet| vec![Self::write(connection_id, packet.compose().get())])
            .unwrap_or_default()
    }

    pub fn plan_all(
        approvals: &[PlayerNameApproval],
        connection_id: i32,
    ) -> Vec<PlayerNetworkEffect> {
        approvals
            .iter()
            .flat_map(|approval| Self::plan(*approval, connection_id))
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
    fn maps_approved_name_to_current_connection_packet() {
        let effects = PlayerNameApprovalNetworkPlan::plan(PlayerNameApproval::Approved, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#NAME_APPROVED##".to_owned(),
            }]
        );
    }

    #[test]
    fn maps_unacceptable_name_to_current_connection_packet() {
        let effects = PlayerNameApprovalNetworkPlan::plan(PlayerNameApproval::Unacceptable, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#NAME_UNACCEPTABLE##".to_owned(),
            }]
        );
    }
}
