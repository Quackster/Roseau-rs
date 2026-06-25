use crate::game::navigator::NavigatorSearchOutcome;
use crate::messages::OutgoingMessage;
use crate::server::PlayerNetworkEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NavigatorSearchNetworkPlan;

impl NavigatorSearchNetworkPlan {
    pub fn plan(outcome: &NavigatorSearchOutcome, connection_id: i32) -> Vec<PlayerNetworkEffect> {
        let mut response = outcome.busy_flat_results().compose();
        vec![PlayerNetworkEffect::WriteResponse {
            connection_id,
            packet: response.get(),
        }]
    }

    pub fn plan_all(
        outcomes: &[NavigatorSearchOutcome],
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
    use crate::game::navigator::NavigatorRequest;
    use crate::game::room::settings::RoomType;
    use crate::game::room::{RoomData, RoomSummary};

    fn room(id: i32, owner_name: &str, show_owner_name: bool, player_count: usize) -> RoomSummary {
        let mut room = RoomSummary::new(RoomData::new(
            id,
            false,
            RoomType::Private,
            7,
            owner_name,
            format!("Room {id}"),
            1,
            "",
            25,
            "A room",
            "model",
            "class",
            "wall",
            "floor",
            false,
            show_owner_name,
        ));
        room.set_player_count(player_count);
        room
    }

    #[test]
    fn maps_search_outcome_to_current_connection_packet() {
        let outcome = NavigatorSearchOutcome::new(
            NavigatorRequest::SearchRooms,
            [room(12, "alice", true, 3)],
            "127.0.0.1",
            37119,
        );

        let effects = NavigatorSearchNetworkPlan::plan(&outcome, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#BUSY_FLAT_RESULTS 1\r12/Room 12/alice/closed//floor1/127.0.0.1/127.0.0.1/37119/3/null/A room##".to_owned(),
            }]
        );
    }

    #[test]
    fn maps_empty_search_outcome_to_java_fallback_packet() {
        let outcome = NavigatorSearchOutcome::empty(NavigatorRequest::PopularRooms);

        let effects = NavigatorSearchNetworkPlan::plan(&outcome, 42);

        assert_eq!(
            effects,
            vec![PlayerNetworkEffect::WriteResponse {
                connection_id: 42,
                packet: "#BUSY_FLAT_RESULTS 1##".to_owned(),
            }]
        );
    }
}
