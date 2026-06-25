use crate::dao::{DaoError, NavigatorDao, RoomDao};
use crate::game::navigator::{NavigatorCommandExecutor, NavigatorSearchOutcome};
use crate::game::player::PlayerManager;
use crate::game::room::RoomManager;
use crate::messages::IncomingExecutionEffect;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NavigatorIncomingPlan;

impl NavigatorIncomingPlan {
    pub fn plan(
        effect: &IncomingExecutionEffect,
        navigator_dao: &dyn NavigatorDao,
        room_dao: &dyn RoomDao,
        room_manager: &RoomManager,
        player_manager: &PlayerManager,
        server_ip: &str,
        private_server_port: u16,
    ) -> Result<Vec<NavigatorSearchOutcome>, DaoError> {
        Ok(NavigatorCommandExecutor::execute(
            effect,
            navigator_dao,
            room_dao,
            room_manager,
            player_manager,
            server_ip,
            private_server_port,
        )?
        .into_iter()
        .collect())
    }

    pub fn plan_all(
        effects: &[IncomingExecutionEffect],
        navigator_dao: &dyn NavigatorDao,
        room_dao: &dyn RoomDao,
        room_manager: &RoomManager,
        player_manager: &PlayerManager,
        server_ip: &str,
        private_server_port: u16,
    ) -> Result<Vec<NavigatorSearchOutcome>, DaoError> {
        let mut outcomes = Vec::new();

        for effect in effects {
            outcomes.extend(Self::plan(
                effect,
                navigator_dao,
                room_dao,
                room_manager,
                player_manager,
                server_ip,
                private_server_port,
            )?);
        }

        Ok(outcomes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::in_memory::{InMemoryNavigatorDao, InMemoryRoomDao};
    use crate::game::navigator::NavigatorRequest;
    use crate::game::player::{PlayerDetails, PlayerSession};
    use crate::game::room::settings::RoomType;
    use crate::game::room::{RoomData, RoomSummary};
    use crate::messages::OutgoingMessage;

    fn private_room(id: i32, owner_id: i32, owner_name: &str, name: &str) -> RoomData {
        RoomData::new(
            id,
            false,
            RoomType::Private,
            owner_id,
            owner_name,
            name,
            0,
            "",
            25,
            "desc",
            "model",
            "class",
            "wall",
            "floor",
            false,
            true,
        )
    }

    fn details(id: i32, username: &str) -> PlayerDetails {
        let mut details = PlayerDetails::new();
        details.fill_basic(id, username, "mission", "figure");
        details
    }

    fn packet(outcome: &NavigatorSearchOutcome) -> String {
        let mut response = outcome.busy_flat_results().compose();
        response.get()
    }

    #[test]
    fn plans_search_flat_effect_through_navigator_dao() {
        let navigator_dao = InMemoryNavigatorDao::new([private_room(10, 7, "alice", "Cafe")]);
        let room_dao = InMemoryRoomDao::new();
        let room_manager = RoomManager::new();
        let player_manager = PlayerManager::new(vec![]);

        let outcomes = NavigatorIncomingPlan::plan(
            &IncomingExecutionEffect::SearchFlat {
                query: "caf".to_owned(),
            },
            &navigator_dao,
            &room_dao,
            &room_manager,
            &player_manager,
            "127.0.0.1",
            37119,
        )
        .unwrap();

        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].request(), NavigatorRequest::SearchRooms);
        assert!(packet(&outcomes[0]).contains("Cafe"));
    }

    #[test]
    fn plans_busy_and_empty_popular_room_effects() {
        let navigator_dao = InMemoryNavigatorDao::new([]);
        let room_dao = InMemoryRoomDao::new();
        let mut room_manager = RoomManager::new();
        let mut loaded = RoomSummary::new(private_room(20, 8, "bob", "Loaded"));
        loaded.set_player_count(2);
        room_manager.add(loaded);
        let player_manager = PlayerManager::new(vec![]);

        let outcomes = NavigatorIncomingPlan::plan_all(
            &[
                IncomingExecutionEffect::SearchBusyFlats { multiplier: 0 },
                IncomingExecutionEffect::EmptySearchBusyFlats,
            ],
            &navigator_dao,
            &room_dao,
            &room_manager,
            &player_manager,
            "127.0.0.1",
            37119,
        )
        .unwrap();

        assert_eq!(outcomes.len(), 2);
        assert_eq!(outcomes[0].request(), NavigatorRequest::PopularRooms);
        assert_eq!(outcomes[0].rooms()[0].data().id(), 20);
        assert_eq!(packet(&outcomes[1]), "#BUSY_FLAT_RESULTS 1##");
    }

    #[test]
    fn plans_online_user_room_search_effect() {
        let navigator_dao = InMemoryNavigatorDao::new([]);
        let room_dao = InMemoryRoomDao::new();
        room_dao.insert_room(private_room(30, 9, "Alice", "Owned"));
        let room_manager = RoomManager::new();
        let mut player_manager = PlayerManager::new(vec![]);
        player_manager.insert(PlayerSession::new(70, 30000, details(9, "Alice")));

        let outcomes = NavigatorIncomingPlan::plan(
            &IncomingExecutionEffect::SearchFlatForUser {
                username: "alice".to_owned(),
            },
            &navigator_dao,
            &room_dao,
            &room_manager,
            &player_manager,
            "10.0.0.1",
            37119,
        )
        .unwrap();

        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].request(), NavigatorRequest::PrivateRooms);
        assert_eq!(outcomes[0].rooms()[0].data().id(), 30);
        assert!(packet(&outcomes[0]).contains("10.0.0.1"));
    }

    #[test]
    fn ignores_missing_users_and_unrelated_effects() {
        let navigator_dao = InMemoryNavigatorDao::new([]);
        let room_dao = InMemoryRoomDao::new();
        let room_manager = RoomManager::new();
        let player_manager = PlayerManager::new(vec![]);

        assert!(NavigatorIncomingPlan::plan(
            &IncomingExecutionEffect::SearchFlatForUser {
                username: "missing".to_owned(),
            },
            &navigator_dao,
            &room_dao,
            &room_manager,
            &player_manager,
            "127.0.0.1",
            37119,
        )
        .unwrap()
        .is_empty());
        assert!(NavigatorIncomingPlan::plan(
            &IncomingExecutionEffect::GoAway,
            &navigator_dao,
            &room_dao,
            &room_manager,
            &player_manager,
            "127.0.0.1",
            37119,
        )
        .unwrap()
        .is_empty());
    }
}
