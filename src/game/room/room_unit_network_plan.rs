use crate::game::room::RoomUnitOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoomUnitNetworkPlan;

impl RoomUnitNetworkPlan {
    pub fn plan(
        outcome: &RoomUnitOutcome,
        connection_id: i32,
        server_ip: &str,
        base_port: i32,
    ) -> Vec<PlayerNetworkEffect> {
        let mut effects = Vec::new();

        if let Some(packet) = outcome.all_units(server_ip, base_port) {
            effects.push(Self::write(connection_id, packet.compose().get()));
        }

        if let Some(packet) = outcome.unit_members_packet() {
            effects.push(Self::write(connection_id, packet.compose().get()));
        }

        effects
    }

    pub fn plan_all(
        outcomes: &[RoomUnitOutcome],
        connection_id: i32,
        server_ip: &str,
        base_port: i32,
    ) -> Vec<PlayerNetworkEffect> {
        outcomes
            .iter()
            .flat_map(|outcome| Self::plan(outcome, connection_id, server_ip, base_port))
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
    use crate::game::room::settings::RoomType;
    use crate::game::room::{RoomData, RoomSummary};

    fn public_room(id: i32, name: &str, class_name: &str, player_count: usize) -> RoomSummary {
        let mut room = RoomSummary::new(RoomData::new(
            id,
            false,
            RoomType::Public,
            -1,
            "",
            name,
            0,
            "",
            25,
            "description",
            "pool_b",
            class_name,
            "wall",
            "floor",
            false,
            true,
        ));
        room.set_player_count(player_count);
        room
    }

    #[test]
    fn maps_unit_listener_to_all_units_packet() {
        let effects = RoomUnitNetworkPlan::plan(
            &RoomUnitOutcome::listener([public_room(5, "Habbo Lido", "lido", 2)]),
            42,
            "127.0.0.1",
            22004,
        );

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#ALLUNITS\rHabbo Lido,2,25,127.0.0.1/127.0.0.1,22009,Habbo Lido\tlido,2,25,pool_b##".to_owned(),
            }]
        );
    }

    #[test]
    fn maps_unit_members_to_all_units_and_member_packets() {
        let effects = RoomUnitNetworkPlan::plan(
            &RoomUnitOutcome::unit_members(
                [public_room(5, "Habbo Lido", "lido", 2)],
                ["alice", "bob"],
            ),
            42,
            "10.0.0.1",
            22004,
        );

        assert_eq!(
            effects,
            vec![
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 42,
                    packet: "#ALLUNITS\rHabbo Lido,2,25,10.0.0.1/10.0.0.1,22009,Habbo Lido\tlido,2,25,pool_b##".to_owned(),
                },
                PlayerNetworkEffect::WriteResponse {
                    connection_id: 42,
                    packet: "#UNITMEMBERS\ralice\rbob##".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn missing_unit_room_has_no_network_effect() {
        assert!(RoomUnitNetworkPlan::plan(
            &RoomUnitOutcome::missing_room(),
            42,
            "127.0.0.1",
            22004
        )
        .is_empty());
    }
}
